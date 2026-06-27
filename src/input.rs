// src/input.rs - Touch, voice, camera passthrough for TheRustyGoat 🐐
use spin::Mutex;

pub static TOUCH_COORDS: Mutex<(i32, i32)> = Mutex::new((0, 0));

pub fn handle_touch(x: i32, y: i32) {
    *TOUCH_COORDS.lock() = (x, y);
    println!("📍 Touch detected at ({}, {}) → forwarding to cloud...", x, y);
    // TODO: Send over QUIC with prediction
    crate::network::send_input(x, y);
}

pub fn handle_voice(audio_data: &[u8]) {
    println!("🎤 Voice captured → sending to cloud for processing (Grok in cloud?)");
    // TODO: Stream to cloud
}

pub fn handle_camera(frame: &[u8]) {
    println!("📸 Camera frame captured → streaming to cloud for AI analysis");
    // TODO: Encode and send
}

pub fn update_local_overlay() {
    // Example: Show local status bar or gesture preview
    println!("🖼️  Updating local overlay on display (battery, connection, etc.)");
}
