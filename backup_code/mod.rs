use chrono::{DateTime, Local};


#[derive(Default,PartialEq)]
pub enum CMGF{
    TEXT,
    #[default]
    PDU
}
impl CMGF {
    pub fn new(text:String)->Self{
        let rest = text.trim().strip_prefix("+CMGF:").expect("Invalid format");
        let cmgf = rest.trim().parse::<u8>().unwrap_or(0);
        match cmgf {
            0=>CMGF::PDU,
            _=>CMGF::TEXT,
        }
    }
}
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
    }pub fn parser(&self)->Vec<String>{
        let mut list = vec![];
        list.push(self.rssi.to_string());
        list.push(self.ber.to_string());
        list.push(self.time.format("%H:%M:%S").to_string());
        list
    }
}


#[derive(Clone)]
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
        let rest = text.trim().strip_prefix("+CESQ:").expect("Invalid CESQ format");
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
    pub fn parser(&self)->Vec<String>{
        let mut list = vec![];
        list.push(self.rxlev.to_string());
        list.push(self.ber.to_string());
        list.push(self.rscp.to_string());
        list.push(self.ecno.to_string());
        list.push(self.rsrq.to_string());
        list.push(self.rsrp.to_string());
        list.push(self.time.format("%H:%M:%S").to_string());
        list
    }
}

pub struct Cnum{
    pub number:String,
    pub n_type:u8,
    pub time:DateTime<Local>,
}
impl Cnum{
    pub fn new(text: String) -> Cnum {
        let mut number = String::new();
        let mut n_type: u8 = 0;
        let t = text.trim().trim_start_matches("+CNUM:").trim();
        let parts: Vec<&str> = t.split('"').collect();
        if parts.len() >= 5 {
            number = parts[3].to_string();
            if let Some(tail) = parts[4].split(',').nth(1) {
                n_type = tail.trim().parse::<u8>().unwrap_or(0);
            }
        }

        Cnum { number, n_type,time: Local::now(), }
    }
    pub fn parser(&self)->Vec<String>{
        vec![self.number.clone(), self.n_type.to_string(),self.time.format("%H:%M:%S").to_string()]
    }
}
pub struct CgpAddr{
    pub ip_addr:String,
    pub time:DateTime<Local>,
}
impl CgpAddr {
    pub fn new(text: String) -> CgpAddr {
        let rest = text.trim().strip_prefix("+CGPADDR: ").expect("Invalid CGPADDR format");
        // let parts: Vec<&str> = rest.trim().split(',').collect();
        CgpAddr{
            ip_addr:rest.to_string(),
            time: Local::now(),
        }
    }
    pub fn parser(&self)->Vec<String>{
        vec![self.ip_addr.clone(), self.time.format("%H:%M:%S").to_string()]
    }
}