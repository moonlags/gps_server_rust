use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::types::PositioningPacket;
use chrono::{Datelike, Timelike};
use tokio::net::TcpStream;
use tracing::info;

#[derive(Debug)]
pub struct DeviceData {
    pub imei: String,
    pub positions: Vec<PositioningPacket>,
    pub battery_power: u8,
    pub last_status_packet_time: i64,
    pub status_cooldown: u8,
    pub is_logged: bool,
    pub is_charging: bool,
    pub connection: TcpStream,
}

impl DeviceData {
    pub fn new(connection: TcpStream) -> DeviceData {
        DeviceData {
            imei: String::new(),
            positions: vec![],
            battery_power: 100,
            last_status_packet_time: chrono::Local::now().timestamp(),
            status_cooldown: 1,
            is_logged: false,
            is_charging: false,
            connection,
        }
    }
    pub async fn handle_connection(
        &self,
        device_session: Arc<Mutex<HashMap<String, &DeviceData>>>,
    ) {
    }
    pub fn process_positioning(
        &mut self,
        positioning: PositioningPacket,
        protocol_number: u8,
    ) -> Vec<u8> {
        info!("new position {:?}", positioning);

        self.positions.push(positioning);
        if self.positions.len() > 10 {
            self.positions.remove(1);
        }

        let time_now = chrono::Local::now();

        vec![
            0x78,
            0x78,
            7,
            protocol_number,
            (time_now.year() - 2000) as u8,
            time_now.month() as u8,
            time_now.day() as u8,
            time_now.hour() as u8,
            time_now.minute() as u8,
            time_now.second() as u8,
            0x0d,
            0x0a,
        ]
    }
}
