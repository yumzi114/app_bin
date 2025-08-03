use std::{cmp::Ordering, collections::BinaryHeap, io::Read, process::exit, sync::Arc, thread::{self}, time::Duration};

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

struct CmdJob {
    priority: u8,
    cmd: String,
    resp_tx: oneshot::Sender<Vec<String>>,
}

impl Eq for CmdJob {}
impl PartialEq for CmdJob {
    fn eq(&self, other: &Self) -> bool { self.priority == other.priority }
}
impl Ord for CmdJob {
    fn cmp(&self, other: &Self) -> Ordering { other.priority.cmp(&self.priority) } // 낮을수록 우선
}
impl PartialOrd for CmdJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

async fn send_cmd(cmd_tx: &mpsc::Sender<(String, oneshot::Sender<Vec<String>>)>, cmd: &str) {
    let (resp_tx, resp_rx) = oneshot::channel();
    cmd_tx.send((cmd.to_string(), resp_tx)).await.unwrap();
    if let Ok(resp) = resp_rx.await {
        println!("RESP[{}]: {:?}", cmd, resp);
    }
}



pub fn lte_sender_thread(app_rx: Receiver<String>) {
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let port = env!("LTE_PORT");
            let lte_baudrate: u32 = env!("LTE_BAUDRATE").parse().unwrap();
            let mut serial = tokio_serial::new(port, lte_baudrate)
                .open_native_async()
                .unwrap();
            #[cfg(unix)]
            serial.set_stop_bits(tokio_serial::StopBits::One).unwrap();
            serial.set_exclusive(false).unwrap();

            let (mut tx, mut rx) = Framed::new(serial, LTELineCodec).split();
            let (cmd_tx, mut cmd_rx) = mpsc::channel::<(u8, String, oneshot::Sender<Vec<String>>)>(20);
            let queue = Arc::new(Mutex::new(BinaryHeap::new()));

            // 명령 요청 수신 → 큐에 적재
            let queue_writer = queue.clone();
            tokio::spawn(async move {
                while let Some((prio, cmd, resp_tx)) = cmd_rx.recv().await {
                    queue_writer.lock().await.push(CmdJob { priority: prio, cmd, resp_tx });
                }
            });

            // 파이프라인화된 송신 & 응답 처리
            let queue_runner = queue.clone();
            // let tx_runner = tx.clone();
            tokio::spawn(async move {
                let mut pending_resp: Option<oneshot::Sender<Vec<String>>> = None;
                let mut resp_buffer = Vec::new();

                loop {
                    tokio::select! {
                        // LTE 응답 수신
                        line = rx.next() => {
                            if let Some(Ok(resp)) = line {
                                resp_buffer.push(resp.clone());
                                if resp.contains("OK") || resp.contains("ERROR") {
                                    if let Some(resp_tx) = pending_resp.take() {
                                        let _ = resp_tx.send(resp_buffer.clone());
                                    }
                                    resp_buffer.clear();
                                }
                            }
                        }

                        // 새로운 명령 실행
                        _ = async {
                            if pending_resp.is_none() {
                                if let Some(job) = queue_runner.lock().await.pop() {
                                    tx.send(job.cmd.clone()).await.unwrap();
                                    pending_resp = Some(job.resp_tx);
                                }
                            }
                        } => {}
                    }
                }
            });

            // 주기적 명령 병합 태스크
            let cmd_tx_clone = cmd_tx.clone();
            tokio::spawn(async move {
                let periodic_cmds = vec!["AT+CSQ", "AT+CESQ", "AT+CMGF?", "AT+CPMS?"];
                loop {
                    for &cmd in &periodic_cmds {
                        let (resp_tx, _resp_rx) = oneshot::channel();
                        cmd_tx_clone.send((5, cmd.to_string(), resp_tx)).await.unwrap();
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // 주기 전체 루프
                }
            });

            // 앱 명령 (최우선)
            let cmd_tx_clone = cmd_tx.clone();
            loop {
                if let Ok(msg) = app_rx.try_recv() {
                    let (resp_tx, _resp_rx) = oneshot::channel();
                    cmd_tx_clone.send((0, msg, resp_tx)).await.unwrap(); // 우선순위 0
                }
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            }
        });
    });
}