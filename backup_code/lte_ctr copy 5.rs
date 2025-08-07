use std::{io::Read, process::exit, thread::{self}, time::Duration};

use chrono::{DateTime, Local};
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use regex::Regex;
use tokio::{io::{split, AsyncWriteExt}, runtime::Runtime, sync::{mpsc, oneshot::{self, channel, Receiver as T_Receiver, Sender as T_Sender}}, time::{sleep, Instant}};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
#[cfg(unix)]
use tokio_serial::{SerialPort, StopBits};
use tokio_util::codec::{Decoder, Framed};
use crate::ctr_mod::{lte_ctr::lte_cmd::{Cesq, CgpAddr, Cnum, Csq, CMGF}, LTELineCodec};
pub mod lte_cmd;

pub struct Lte_Reader_Task{
    pub msg_tx:Sender<String>,
    pub msg_rx:Receiver<String>,
    pub last_csq:Option<Csq>,
    pub last_cesq:Option<Cesq>,
    pub last_cgpaddr:Option<CgpAddr>,
    pub last_cnum:Option<Cnum>,
    pub network_timeover:u8,
    pub cmgf:CMGF,
    pub app_tx:Sender<String>,
    pub app_rx:Receiver<String>
}
impl Lte_Reader_Task{
    pub fn new()->Self{
        let (msg_tx,msg_rx)=unbounded::<String>();
        let last_csq=None;
        let last_cesq=None;
        let last_cnum=None;
        let last_cgpaddr=None;
        let network_timeover=3;
        let cmgf = CMGF::default();
        let (app_tx, app_rx) = unbounded::<String>();
        // let
        Self { 
            msg_tx,msg_rx,last_csq,last_cesq,last_cnum,last_cgpaddr,network_timeover,cmgf,app_tx, app_rx
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
    app_rx:Receiver<String>
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
            let mut last_activity = Instant::now();
            
            let writer_task = async move {
                while let Some((cmd, resp_tx)) = cmd_rx.recv().await {
                    tx.send(cmd).await.unwrap();
                    last_activity = Instant::now();

                }
            };
            tokio::spawn(writer_task);
            

            let cmd_tx_clone = cmd_tx.clone();
            tokio::spawn(async move {
                loop {
                    if last_activity.elapsed() > Duration::from_secs(2) {
                        send_cmd(&cmd_tx_clone, "AT+CNUM").await;
                    };
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
            });
            let cmd_tx_clone = cmd_tx.clone();
            tokio::spawn(async move {
                loop {
                    if last_activity.elapsed() > Duration::from_secs(2) {
                        send_cmd(&cmd_tx_clone, "AT+CGPADDR=1").await;
                    }
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
            });
            let cmd_tx_clone = cmd_tx.clone();
            tokio::spawn(async move {
                loop {
                    if last_activity.elapsed() > Duration::from_secs(2) {
                        send_cmd(&cmd_tx_clone, "AT+CESQ").await;
                    };
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            });
            let cmd_tx_clone = cmd_tx.clone();
            tokio::spawn(async move {
                loop {
                    if last_activity.elapsed() > Duration::from_secs(2) {
                        send_cmd(&cmd_tx_clone, "AT+CSQ").await;
                    };
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            });
            let cmd_tx_clone = cmd_tx.clone();
            tokio::spawn(async move {
                loop {
                    if last_activity.elapsed() > Duration::from_secs(2) {
                        send_cmd(&cmd_tx_clone, "AT+CMGF?").await;
                    };
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            });
            // tokio::spawn(async move {
            //     loop {
            //         if last_activity.elapsed() > Duration::from_secs(2) {
            //             send_cmd(&cmd_tx_clone, "AT+CNUM").await;
            //             send_cmd(&cmd_tx_clone, "AT+CGPADDR=1").await;
            //             //문자
            //             send_cmd(&cmd_tx_clone, "AT+CPMS?").await;
            //         };
                    
            //         tokio::time::sleep(Duration::from_secs(10)).await;
            //     }
            // });
            
            // let cmd_tx_clone = cmd_tx.clone();
            // tokio::spawn(async move {
            //     loop {
            //         if last_activity.elapsed() > Duration::from_secs(2) {
            //             send_cmd(&cmd_tx_clone, "AT+CESQ").await;
            //             send_cmd(&cmd_tx_clone, "AT+CSQ").await;
            //             //모드확인
            //             send_cmd(&cmd_tx_clone, "AT+CMGF?").await;
            //         };
                    
            //         tokio::time::sleep(Duration::from_secs(1)).await;
            //     }
            // });
            
            let cmd_tx_clone = cmd_tx.clone();
            loop{
                if let Ok(msg) = app_rx.try_recv() {
                    send_cmd(&cmd_tx_clone, &msg).await;
                    // cmd_tx_clone.send(msg).await.unwrap();
                }
                // sleep(Duration::from_millis(1)).await; 
                // if let Ok(_)=serial.write_all(b"AT\r\n").await{
                //     // msg_tx.send("OK Send".to_string()).unwrap();
                // };
            }

        });
    });
}