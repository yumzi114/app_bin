use std::{cmp::Ordering, collections::BinaryHeap, io::Read, process::exit, sync::{Arc}, thread::{self}, time::Duration};

use chrono::{DateTime, Local};
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use regex::Regex;
use tokio::{io::{split, AsyncWriteExt}, runtime::Runtime, sync::{mpsc, oneshot::{self, channel, Receiver as T_Receiver, Sender as T_Sender}, Mutex}, time::sleep};
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

// #[derive(Eq)]
struct CmdJob {
    priority: u8,
    cmd: String,
    resp_tx: oneshot::Sender<Vec<String>>,
}
impl Eq for CmdJob {}

impl PartialEq for CmdJob {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Ord for CmdJob {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority) // 낮은 숫자가 높은 우선순위
    }
}

impl PartialOrd for CmdJob {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
            // let (line_tx, mut line_rx) = mpsc::unbounded_channel::<String>();
            let (cmd_tx, mut cmd_rx) = mpsc::channel::<(u8,String, oneshot::Sender<Vec<String>>)>(10);
            let queue = Arc::new(Mutex::new(BinaryHeap::new()));
            let queue_writer = queue.clone();
            tokio::spawn(async move {
                while let Some((prio, cmd, resp_tx)) = cmd_rx.recv().await {
                    queue_writer.lock().await.push(CmdJob { priority: prio, cmd, resp_tx });
                }
            });
            let queue_sender = queue.clone();
            let writer_task=async move {
                loop {
                    if let Some(job) = queue_sender.lock().await.pop() {
                        tx.send(job.cmd.clone()).await.unwrap();
                        // 응답은 여기서 별도 처리 가능 (수신부 필요 시)
                    } else {
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    }
                }
            };
            tokio::spawn(writer_task);
            let cmd_tx_clone = cmd_tx.clone();
            async fn send_cmd(
                cmd_tx: &mpsc::Sender<(u8, String, oneshot::Sender<Vec<String>>)>,
                priority: u8,
                cmd: &str,
            ) {
                let (resp_tx, _resp_rx) = oneshot::channel();
                cmd_tx.send((priority, cmd.to_string(), resp_tx)).await.unwrap();
            }
            let periodic_cmds = vec![
                ("AT+CNUM", 10),
                ("AT+CGPADDR=1", 10),
                ("AT+CESQ", 1),
                ("AT+CSQ", 1),
                ("AT+CMGF?", 1),
                ("AT+CPMS?", 2),
            ];
            for (cmd, interval) in periodic_cmds {
                let cmd_tx_clone = cmd_tx.clone();
                tokio::spawn(async move {
                    loop {
                        send_cmd(&cmd_tx_clone, 5, cmd).await; // 우선순위 5 (낮음)
                        tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
                    }
                });
            }
            
            loop {
                if let Ok(msg) = app_rx.try_recv() {
                    send_cmd(&cmd_tx_clone, 0, &msg).await; // 우선순위 0
                }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }

        });
    });
}