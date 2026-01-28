use std::{sync::Arc, time::Duration};

use axum::{
    extract::ws::{WebSocketUpgrade, Message},
};
use tokio::{sync::Mutex, time::sleep};
use once_cell::sync::Lazy;
use futures_util::{sink::SinkExt, stream::StreamExt};

pub static LAST_FRAME: Lazy<Arc<Mutex<Option<Vec<u8>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl axum::response::IntoResponse {
    ws.on_upgrade(|socket| async move {
        let (mut sender, _) = socket.split();
        let last_frame = LAST_FRAME.clone();

        loop {
            {
                let frame_guard = last_frame.lock().await;
                if let Some(frame) = &*frame_guard {
                    if sender.send(Message::Binary(frame.clone())).await.is_err() {
                        eprintln!("Error when sending frame: connection closed");
                        break;
                    }
                }
            }
            sleep(Duration::from_millis(33)).await; // ~30 FPS
        }
    })
}