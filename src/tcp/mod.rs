use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::net::TcpListener;
use tracing::info;

use self::device_data::DeviceData;

pub mod device_data;
pub mod types;

pub async fn tcp_handler(device_sessions: Arc<Mutex<HashMap<String, &'static DeviceData>>>) {
    let listener = TcpListener::bind("127.0.0.1:55080").await.unwrap();

    loop {
        if let Ok((socket, addr)) = listener.accept().await {
            info!("connection from {addr}");

            let device_sessions = device_sessions.clone();

            tokio::spawn(async move {
                let device_data = DeviceData::new(socket);

                device_data.handle_connection(device_sessions).await;
            });
        }
    }
}
