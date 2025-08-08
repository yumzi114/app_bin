//UART READ FORMATING 


use std::io;

use bytes::Buf;
use bytes::BufMut;
use bytes::BytesMut;
use tokio_util::codec::Decoder;
use tokio_util::codec::Encoder;
use ucs2::decode as ucs2_decode;
use ublox::PacketRef;
use ublox::{Parser};

use crate::ctr_mod::board_ctl::App_Protocol;
// use nmea::Nmea;
// pub mod gps_ctr;
// pub mod lte_ctr;
pub mod board_ctl;




pub struct BoardLineCodec;

impl Decoder for BoardLineCodec {
    type Item = App_Protocol;
    type Error = io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let packet_size = core::mem::size_of::<App_Protocol>();

        let newline = src.as_ref().iter().position(|b| *b == 0xFC);
        if let Some(pos) = newline {
            if pos + 1 < packet_size {
                // 너무 짧음: 쓰레기 → sync 복구
                src.advance(pos + 1);
                return Ok(None);
            }
            let line = src.split_to(pos + 1);
            if line.len() != packet_size {
                // 크기 안 맞음 → sync 복구
                return Ok(None);
            }
            if line[0] == 0xAF {
                let pkt = bytemuck::from_bytes::<App_Protocol>(&line[..]);
                if pkt.verify_crc() {
                    return Ok(Some(*pkt));
                }
            }
            src.advance(1);

            // let line = src.split_to(n + 1);
            // let line_list = line.to_vec();
            // if line_list[0]==0xAF&&line_list[2]==0x1B&&line_list.len() == core::mem::size_of::<App_Protocol>(){
            //     let pkt = bytemuck::from_bytes::<App_Protocol>(&line_list[..]);
            //     if pkt.verify_crc(){
            //         // let text = format!("{:?}",pkt);
            //         return Ok(Some(pkt.clone()));
            //     }
            // }
        }
        Ok(None)
    }
}

impl Encoder<String> for BoardLineCodec {
    type Error = io::Error;

    fn encode(&mut self, _item: String, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

// pub struct GPSLineCodec;

// impl Decoder for GPSLineCodec {
//     type Item = String;
//     type Error = io::Error;

//     fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
//         let newline = src.as_ref().iter().position(|b| *b == b'\n');
//         if let Some(n) = newline {
//             let line = src.split_to(n + 1);
//             let line_list = line.to_vec();
//             if line_list[0]==b'$' {
//                 return match str::from_utf8(line.as_ref()) {
//                     Ok(s) => Ok(Some(s.to_string())),
//                     Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
//                 };
//             }
//         }
//         Ok(None)
//     }
// }
// impl Encoder<String> for GPSLineCodec {
//     type Error = io::Error;

//     fn encode(&mut self, _item: String, _dst: &mut BytesMut) -> Result<(), Self::Error> {
//         Ok(())
//     }
// }


// pub struct LTELineCodec;

// impl Decoder for LTELineCodec {
//     type Item = String;
//     type Error = io::Error;

//     fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
//         if let Some(n) = src.iter().position(|b| *b == b'\n') {
//             let mut line = src.split_to(n + 1);

//             // CRLF 제거
//             if line.ends_with(b"\r\n") {
//                 line.truncate(line.len() - 2);
//             } else if line.ends_with(b"\n") {
//                 line.truncate(line.len() - 1);
//             }
//             // let decoded: String = ucs2_decode(&ucs2_bytes).collect();
//             match std::str::from_utf8(&line) {
//                 Ok(s) => {
//                     let trimmed = s.trim();
//                     if trimmed.is_empty() {
//                         Ok(None) // 빈 줄은 무시
//                     } else {
//                         // let trimmed= trimmed.to_string().replace("OK", "");
//                         if trimmed.is_empty() {
//                             Ok(None) // ✅ OK만 있거나 제거 후 빈 문자열이면 무시
//                         } else {
//                             Ok(Some(trimmed.to_string()))
//                         }
//                         // Ok(Some(trimmed.to_string()))
//                     }
//                 }
//                 Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")),
//             }
//         } else {
//             Ok(None)
//         }
//     }
// }

// impl Encoder<String> for LTELineCodec {
//     type Error = io::Error;

//     fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
//         // 송신 시 자동으로 CRLF 붙여주기
//         dst.extend_from_slice(item.as_bytes());
//         if !item.ends_with("\r\n") {
//             dst.extend_from_slice(b"\r\n");
//         }
//         Ok(())
//     }
// }


