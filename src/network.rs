// src/network.rs
#![allow(unused)]

use spin::Mutex;

pub static CLOUD_CONNECTION: Mutex<CloudSession> = Mutex::new(CloudSession::Disconnected);

pub enum CloudSession {
    Disconnected,
    Connecting,
    Connected { session_id: u64 }, // placeholder
}

pub fn connect_to_cloud() {
    println!("🌐 Connecting to cloud instance via QUIC + TLS...");
    // TODO: Add quinn or rustls + your cloud endpoint
    *CLOUD_CONNECTION.lock() = CloudSession::Connected { session_id: 1337 };
    println!("✅ Connected! Session 1337 active.");
    
    start_stream_receiver();
}

fn start_stream_receiver() {
    println!("📺 Starting AV1/H.265 decoder + compositor thread...");
    // TODO: Hardware decode + copy frames to framebuffer
    println!("🎥 Streaming live from cloud... (mock frame received)");
}

pub fn send_input(touch_x: u32, touch_y: u32) {
    // TODO: Send over QUIC with <10ms latency + prediction
    println!("📍 Input forwarded → cloud: ({}, {})", touch_x, touch_y);
}

// In main.rs you can call: network::connect_to_cloud();
