use std::{cmp::Ordering, collections::BinaryHeap, io::Read, process::exit, sync::Arc, thread::{self}, time::Duration};

use chrono::{DateTime, Local};
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use regex::Regex;
use tokio::{io::{split, AsyncWriteExt}, runtime::Runtime, sync::{mpsc, oneshot::{self, channel, Receiver as T_Receiver, Sender as T_Sender}, Mutex}, time::{sleep, Instant}};
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
            // 시리얼 포트 열기
            let port = env!("LTE_PORT");
            let lte_baudrate: u32 = env!("LTE_BAUDRATE").parse().unwrap();
            let mut serial = tokio_serial::new(port, lte_baudrate)
                .open_native_async()
                .unwrap();
            #[cfg(unix)]
            serial.set_stop_bits(tokio_serial::StopBits::One).unwrap();
            serial.set_exclusive(false).unwrap();

            let (mut tx, mut rx) = Framed::new(serial, LTELineCodec).split();

            // 명령 전송 채널
            let (cmd_tx, mut cmd_rx) = mpsc::channel::<(String, oneshot::Sender<Vec<String>>)>(20);

            let cmd_tx_clone = cmd_tx.clone();
            // 명령 송신 task
            tokio::spawn(async move {
                let mut pending_resp: Option<oneshot::Sender<Vec<String>>> = None;
                let mut resp_buffer: Vec<String> = Vec::new();
                let mut last_activity = Instant::now();

                loop {
                    tokio::select! {
                        // LTE 응답 수신
                        line = rx.next() => {
                            if let Some(Ok(resp)) = line {
                                resp_buffer.push(resp.clone());
                                if resp.contains("OK") || resp.contains("ERROR") {
                                    if let Some(tx) = pending_resp.take() {
                                        let _ = tx.send(resp_buffer.clone());
                                    }
                                    resp_buffer.clear();
                                    last_activity = Instant::now(); // 응답 도착 → 활동 시간 갱신
                                }
                            }
                        }

                        // 앱 명령/큐 명령 전송
                        Some((cmd, resp_tx)) = cmd_rx.recv() => {
                            tx.send(cmd.clone()).await.unwrap();
                            pending_resp = Some(resp_tx);
                            last_activity = Instant::now();
                        }

                        // Idle 감지: 2초 이상 활동 없으면 상태 명령 실행
                        _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                            if last_activity.elapsed() > std::time::Duration::from_secs(1) && pending_resp.is_none() {
                                // Idle 상태 → 상태 명령 전송
                                for status_cmd in ["AT+CSQ", "AT+CESQ", "AT+CMGF?", "AT+CPMS?"] {
                                    let (resp_tx, _resp_rx) = oneshot::channel();
                                    cmd_tx_clone.send((status_cmd.to_string(), resp_tx)).await.unwrap();
                                }
                                last_activity = Instant::now();
                            }
                        }
                    }
                }
            });

            // 앱 명령 수신 루프
            let cmd_tx_clone = cmd_tx.clone();
            loop {
                if let Ok(msg) = app_rx.try_recv() {
                    let (resp_tx, _resp_rx) = oneshot::channel();
                    cmd_tx_clone.send((msg, resp_tx)).await.unwrap();
                }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        });
    });
}