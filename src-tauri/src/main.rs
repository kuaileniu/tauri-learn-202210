#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{thread, time};
use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct MyPayload {
    message: String,
    ptype: i32,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("rust greet");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn args_command(a: f32, b: f32) -> f32 {
    println!("I was invoked from JS, with this message: {} {}", a, b);
    let c = a * b;
    c
}

#[tauri::command]
fn init_process(window: Window) {
    std::thread::spawn(move || loop {
        window
            .emit(
                "event-name-hggfjt",
                MyPayload {
                    message: "Tauri is awesome!".into(),
                    ptype: 8,
                },
            )
            .unwrap();
        thread::sleep(time::Duration::from_secs(2));
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, args_command, init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
