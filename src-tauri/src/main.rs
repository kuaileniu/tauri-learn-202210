#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{thread, time};
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::Window;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

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

#[tauri::command]
fn read_every_text_file(path: std::path::PathBuf) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(show);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            args_command,
            init_process,
            read_every_text_file
        ])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    dbg!("系统quit退出");
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        // .run(tauri::generate_context!())
        // .expect("error while running tauri application")

        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                // dbg!("阻止退出,系统仍在后台运行");
                println!("界面退出,系统仍在后台运行");
                api.prevent_exit(); //阻止退出,在后台运行
            }
            _ => {}
        });
}
