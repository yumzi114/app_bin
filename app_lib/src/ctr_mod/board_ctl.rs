use std::thread;

use crc::{Algorithm, Crc};
use bytemuck::{bytes_of, from_bytes, Pod, Zeroable};
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures_util::StreamExt;
use tokio::runtime::Runtime;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Framed;

use crate::ctr_mod::BoardLineCodec;





const CRC_8_MAXIM: Algorithm<u8> = Algorithm {
    width: 8,             // CRC 길이 (비트) → 8비트
    poly: 0x31,           // 다항식 (x⁸ + x⁵ + x⁴ + 1)
    init: 0x00,           // 초기값 (시작 CRC 값)
    refin: true,          // 입력 바이트를 LSB 기준으로 반전할지 여부
    refout: true,         // 출력 CRC를 비트 반전할지 여부
    xorout: 0x00,         // 최종 CRC에 XOR 연산할 값
    check: 0xA1,          // `"123456789"` 입력에 대한 expected CRC → 검증용
    residue: 0x00,        // 정상적으로 전송/수신되었을 때의 CRC 잔여값
};


pub struct Board_task{
    pub protocol:App_Protocol,
    pub protocol_tx:Sender<App_Protocol>,
    pub protocol_rx:Receiver<App_Protocol>,
}
impl Board_task {
    pub fn new()->Board_task{
        let (protocol_tx,protocol_rx)=unbounded::<App_Protocol>();
        let protocol =App_Protocol::new();
        Board_task{
            protocol,
            protocol_tx,
            protocol_rx
        }
    }
}

#[repr(C, packed)]
#[derive(Default,Debug,Clone, Copy, Pod, Zeroable)]
pub struct App_Protocol{
    //LTE FIELD
    start:u8,
    cmd:u8,
    len:u8,
    pub lte_state:u8,
    pub rssi:u8,
    pub rsrq:u8,
    pub rsrp:u8,
    ip:u32,
    ip_pid:u8,
    // phone_num:u64,
    // phone_num_type:u8,
    cpms:u8,
    pub gps_lat:f32,
    pub gps_lon:f32,
    checksum:u8,
    end:u8
}
impl App_Protocol{
    pub fn new()->App_Protocol{
        App_Protocol{
            start:0xAF,
            cmd:0xFD,
            len:0x13, //len ~ gps_lon byte 24byte
            end:0xFC,
            ..Default::default()
        }
    }
    pub fn check_update(&mut self){
        let crc = Crc::<u8>::new(&CRC_8_MAXIM);
        let raw = bytemuck::bytes_of(self);
        let result = crc.checksum(&raw[2..21]);//LEN ~ gps_lon
        self.checksum=result;
    }

    pub fn verify_crc(&self) -> bool {
        let crc = Crc::<u8>::new(&CRC_8_MAXIM);
        let raw = bytemuck::bytes_of(self);
        let calc = crc.checksum(&raw[2..21]);
        calc == self.checksum
    }
    fn parse_packet(buf: &[u8]) -> Option<App_Protocol> {
        if buf.len() != core::mem::size_of::<App_Protocol>() {
            return None;
        }
    
        Some(*from_bytes::<App_Protocol>(buf))
    }
}


pub fn board_reader_thread(
    protocol_tx:Sender<App_Protocol>,
){
    thread::spawn(move||{
        let rt  = Runtime::new().unwrap();
        rt.block_on(async move{
            let port =env!("BOARD_PORT");
            let board_baudrate:u32 =env!("BOARD_BAUDRATE").parse().unwrap();
            let mut serial = tokio_serial::new(port, board_baudrate)
                .open_native_async().unwrap(); 
            #[cfg(unix)]
            serial.set_exclusive(false)
                .expect("Unable to set serial port exclusive to false");
            let mut framed = Framed::new(serial, BoardLineCodec);
            
            // let (reader, mut writer) = LTELineCodec.framed(serial).split(); 
            loop{
                if let Some(Ok(pro))=framed.next().await{
                    // println!("OKKKKK");
                    protocol_tx.send(pro).unwrap();
                }
            } 
        });
    });
}