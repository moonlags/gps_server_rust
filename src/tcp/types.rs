#[derive(Debug)]
pub struct PositioningPacket {
    pub latitude: f32,
    pub longitude: f32,
    pub speed: u8,
    pub heading: u16,
    pub timestamp: i64,
}

impl PositioningPacket {
    pub fn new(latitude: f32, longitude: f32, speed: u8, heading: u16) -> PositioningPacket {
        PositioningPacket {
            latitude,
            longitude,
            speed,
            heading,
            timestamp: chrono::Local::now().timestamp(),
        }
    }
}

struct LoginPacket {
    pub imei: String,
}
