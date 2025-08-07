
use std::{ sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread::{self}, time::Duration};

use nmea::Nmea;
use tokio::{runtime::Runtime,time::sleep};
#[cfg(unix)]
use tokio_serial::{SerialPort, StopBits};
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Decoder;
use futures_util::{ StreamExt};
// use crate::ctr_mod::GPSLineCodec;
use crossbeam_channel::{unbounded, Receiver, Sender};
use chrono::{DateTime, Local, TimeZone};
use gpio_cdev::{Chip, LineRequestFlags};
// #[derive(Defaults)]
pub struct Gps_Reader_task{
    pub msg_tx : Sender<String>,
    pub msg_rx : Receiver<String>,
    pub is_running:Arc<AtomicBool>,
    pub nmea : Arc<Mutex<Nmea>>,
    // pub power_pin : Arc<Mutex<gpio_cdev::LineHandle>>,
    pub start_time:Arc<Mutex<Option<DateTime<Local>>>>,
    
}


impl Gps_Reader_task{
    pub fn new()->Self{
        // let mut chip = Chip::new("/dev/gpiochip0").unwrap();
        // let handle: gpio_cdev::LineHandle = chip
        // .get_line(17).unwrap()
        // .request(LineRequestFlags::OUTPUT, 0, "gpio17-control").unwrap();
        // let power_pin = Arc::new(Mutex::new(handle));
        let (msg_tx,msg_rx)=unbounded::<String>();
        // let (closer_tx,closer_rx)=tokio::sync::watch::channel(false);
        let is_running: Arc<AtomicBool>=Arc::new(AtomicBool::new(true));
        let nmea =Arc::new(Mutex::new(Nmea::default()));
        let start_time = Arc::new(Mutex::new(None));
        Gps_Reader_task{
            msg_rx,msg_tx,
            is_running,
            nmea,
            // power_pin,
            start_time
            // closer_rx,closer_tx
        }
    }
}

pub fn gps_reader_thread(
    msg_tx:Sender<String>,
    is_running: Arc<AtomicBool>,
    nmea : Arc<Mutex<Nmea>>,
    // power_pin : Arc<Mutex<gpio_cdev::LineHandle>>,
    start_time: Arc<Mutex<Option<DateTime<Local>>>>
    // closer_rx:tokio::sync::watch::Receiver<bool>,
){
    thread::spawn(move||{
        // let mut nmea: Nmea = Nmea::default();
        let rt  = Runtime::new().unwrap();
        rt.block_on(async move{
            let port =env!("GPS_PORT");
            let gps_baudrate:u32 =env!("GPS_BAUDRATE").parse().unwrap();
            // let mut guard =nmea.lock().unwrap();
            let mut serial = tokio_serial::new(port, gps_baudrate).open_native_async().unwrap();  
            #[cfg(unix)]
            serial.set_stop_bits(StopBits::One).unwrap();
            serial.set_exclusive(false)
                .expect("Unable to set serial port exclusive to false");
            *start_time.lock().unwrap()=Some(Local::now());
            is_running.store(true, Ordering::Release);
            let mut line = GPSLineCodec.framed(serial);
            loop{
                if let Some(Ok(msg))=line.next().await {
                    msg_tx.send(msg.clone()).unwrap();
                    if let Ok(gd)=nmea.lock().as_mut(){
                        gd.parse(&msg).ok();
                    }
                }
            }
        });
    });
}