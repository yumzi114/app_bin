use std::{io::Read, process::exit, thread::{self}, time::Duration};

use chrono::{DateTime, Local};
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use tokio::{io::{split, AsyncWriteExt}, runtime::Runtime, sync::{mpsc, oneshot}, time::sleep};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
#[cfg(unix)]
use tokio_serial::{SerialPort, StopBits};
use tokio_util::codec::{Decoder, Framed};
use crate::ctr_mod::LTELineCodec;


pub struct  Csq {
    pub rssi: u8,
    pub ber:u8,
    pub time:DateTime<Local>,
}
impl Csq {
    pub fn new(text: String) -> Csq {
        let rest = text.trim().strip_prefix("+CSQ:").expect("Invalid CSQ format");
        let parts: Vec<&str> = rest.trim().split(',').collect();
        let rssi = parts[0].trim().parse::<u8>().unwrap_or(255);
        let ber = parts[1].trim().parse::<u8>().unwrap_or(255);
        Csq {
            rssi,
            ber,
            time: Local::now(),
        }
    }
}



pub struct  Cesq {
    pub rxlev: u8, //GSM (2G) 수신 레벨
    pub ber:u8,  //Bit Error Rate 2G 비트오류율
    pub rscp:u8, //UMTS (3G) 신호 세기
    pub ecno:u8, //UMTS (3G) 신호 품질
    pub rsrq:u8, // (-20 ~ -3 dB) LTE품질
    pub rsrp:u8, //-140 ~ -44 dBm LTE 신호 세기
    pub time:DateTime<Local>,
}
impl Cesq {
    pub fn new(text: String) -> Cesq {
        let rest = text.trim().strip_prefix("+CESQ:").expect("Invalid CSQ format");
        let parts: Vec<&str> = rest.trim().split(',').collect();
        // let rssi = parts[0].trim().parse::<u8>().expect("Invalid RSSI value");
        // let ber = parts[1].trim().parse::<u8>().expect("Invalid BER value");
        Cesq {
            rxlev: parts[0].trim().parse().unwrap_or(255),
            ber:   parts[1].trim().parse().unwrap_or(99),
            rscp:  parts[2].trim().parse().unwrap_or(255),
            ecno:  parts[3].trim().parse().unwrap_or(255),
            rsrq:  parts[4].trim().parse().unwrap_or(255),
            rsrp:  parts[5].trim().parse().unwrap_or(255),
            time: Local::now(),
        }
    }
}
pub struct Lte_Reader_Task{
    pub msg_tx:Sender<String>,
    pub msg_rx:Receiver<String>,
    pub last_csq:Option<Csq>,
    pub last_cesq:Option<Cesq>,
}
impl Lte_Reader_Task{
    pub fn new()->Self{
        let (msg_tx,msg_rx)=unbounded::<String>();
        let last_csq=None;
        let last_cesq=None;
        Self { 
            msg_tx,msg_rx,last_csq,last_cesq
         }

    }
}


pub fn lte_reader_thread(
    msg_tx:Sender<String>,
){
    thread::spawn(move||{
        let rt  = Runtime::new().unwrap();
        rt.block_on(async move{
            
            let port =env!("LTE_PORT");
            let lte_baudrate:u32 =env!("LTE_BAUDRATE").parse().unwrap();
            let mut serial = tokio_serial::new(port, lte_baudrate)
                .open_native_async().unwrap();  
            #[cfg(unix)]
            serial.set_stop_bits(StopBits::One).unwrap();
            serial.set_exclusive(false)
                .expect("Unable to set serial port exclusive to false");
            let mut framed = Framed::new(serial, LTELineCodec);
            framed.send("ATE0".to_string()).await.unwrap();
            // let (reader, mut writer) = LTELineCodec.framed(serial).split(); 
            loop{
                if let Some(Ok(msg))=framed.next().await{
                    
                    msg_tx.send(msg).unwrap();
                }
            }
            // 
            
        })
    });
}



async fn send_cmd(cmd_tx: &mpsc::Sender<(String, oneshot::Sender<Vec<String>>)>, cmd: &str) {
    let (resp_tx, resp_rx) = oneshot::channel();
    cmd_tx.send((cmd.to_string(), resp_tx)).await.unwrap();
    if let Ok(resp) = resp_rx.await {
        println!("RESP[{}]: {:?}", cmd, resp);
    }
}



pub fn lte_sender_thread(
    // msg_tx:Sender<String>,
){
    thread::spawn(move||{
        let rt  = Runtime::new().unwrap();
        rt.block_on(async move{
            let port =env!("LTE_PORT");
            let lte_baudrate:u32 =env!("LTE_BAUDRATE").parse().unwrap();
            let mut serial = tokio_serial::new(port, lte_baudrate)
                .open_native_async().unwrap(); 
            #[cfg(unix)]
            serial.set_stop_bits(StopBits::One).unwrap();
            serial.set_exclusive(false).expect("Unable to set serial port exclusive to false"); 
            let (mut tx,rx) = Framed::new(serial, LTELineCodec).split();
            let (line_tx, mut line_rx) = mpsc::unbounded_channel::<String>();
            let (cmd_tx, mut cmd_rx) = mpsc::channel::<(String, oneshot::Sender<Vec<String>>)>(10);

            let writer_task = async move {
                while let Some((cmd, resp_tx)) = cmd_rx.recv().await {
                    tx.send(cmd).await.unwrap();
                }
            };
            tokio::spawn(writer_task);
            let cmd_tx_clone = cmd_tx.clone();
            // tokio::spawn(async move {
            //     loop {
            //         send_cmd(&cmd_tx_clone, "AT").await;
            //         tokio::time::sleep(Duration::from_secs(1)).await;
            //     }
            // });
            // let cmd_tx_clone = cmd_tx.clone();
            // tokio::spawn(async move {
            //     loop {
            //         send_cmd(&cmd_tx_clone, "AT+CREG?").await;
            //         tokio::time::sleep(Duration::from_secs(1)).await;
            //     }
            // });
            // let cmd_tx_clone = cmd_tx.clone();
            // tokio::spawn(async move {
            //     loop {
            //         send_cmd(&cmd_tx_clone, "AT+CSMS?").await;
            //         tokio::time::sleep(Duration::from_secs(1)).await;
            //     }
            // });
            tokio::spawn(async move {
                loop {
                    send_cmd(&cmd_tx_clone, "AT+CESQ").await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            });
            let cmd_tx_clone = cmd_tx.clone();
            tokio::spawn(async move {
                loop {
                    send_cmd(&cmd_tx_clone, "AT+CSQ").await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            });
            loop{
                sleep(Duration::from_secs(60)).await; 
                // if let Ok(_)=serial.write_all(b"AT\r\n").await{
                //     // msg_tx.send("OK Send".to_string()).unwrap();
                // };
            }

        });
    });
}