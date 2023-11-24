use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tcp::device_data::DeviceData;

mod tcp;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().init();

    let device_sessions: Arc<Mutex<HashMap<String, &'static DeviceData>>> =
        Arc::new(Mutex::new(HashMap::new()));

    {
        let device_sessions = device_sessions.clone();

        tokio::spawn(async move {
            tcp::tcp_handler(device_sessions).await;
        });
    }

    Ok(())
}
