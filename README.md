# app_bin
systemctl --user stop app_bin.service

sudo stty -F /dev/ttyAMA2 raw -echo -icanon min 1 time 1

for rasberry
![APP VIEW](https://github.com/user-attachments/assets/adb9b981-bcc4-4d2a-ac70-3d6570de7a7b)
todo!(RF, CAN)
----------------------------------------------
header
---------------------------------------------
STX 1B
AF

CMD 1B 
LTE 0x00 | GPS 0x01 | RF 0x02 | CAN 0x03 | REP 0xFD | RES 0xFE|REQ 0xFF

LEN 1B
DATA LEN

DATA ?B

CHECKSUM 2B

END  1B

6B+@(DATA)
______________________________________________



LTE 
상태플레그 :0000_ LTE모듈ON/OFF | GPS_ON/OFF | EEPROM UPDATE | CMGF MODE 1bit  -1바이트

Csq:rssi - 1바이트
Cesq: rsrp:u8, - 1바이트
Cesq: rsrq:u8, - 1바이트

ip: u32      - 4바이트 LE 
ip_pid: u8   - 1바이트
phone num :u64   - 8바이트 LE
phone_num_type:u8    - 1바이트
cpms: u8     1바이트  신규문자

=19B + 6B
7B ??? MQTT?? REQ ??
'

---------------------------------------------
pub enum CMGF{
    TEXT,
    #[default]
    PDU
}

pub struct  Csq {
    pub rssi: u8,         -1바이트
    pub ber:u8,
    pub time:DateTime<Local>,
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

pub struct Cnum{
    pub number:String,    u64 - LE
    pub n_type:u8,
    pub time:DateTime<Local>,
}

pub struct CgpAddr{
    pub ip_addr:String,
    pub time:DateTime<Local>,
}