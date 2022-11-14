#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use glium::texture::ClientFormat;
use test5::image_encode_base64;
use std::fs::{self, File};
use std::io::Read;
use std::{path, vec, string};
use std::{thread, time};
use tauri::AppHandle;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::Window;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};

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

#[tauri::command]
fn read_img_file(path: &str)-> String {
    image_encode_base64(path)
}

#[tauri::command]
fn read_img_file444(path: &str) {
    // fn read_img_file(path: &path::PathBuf) {
    // println!("path:--->{}", path);
    use png::ColorType::*;
    let f = fs::File::open(path).expect("Something went wrong reading the png file");
    let mut decoder = png::Decoder::new(f);
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().expect("未能成功读取图片信息");
    let mut img_data = vec![0; reader.output_buffer_size()];
    let info = reader.info();//Info { width: 310, height: 310, bit_depth: Eight, color_type: Rgba, interlaced: false, trns: None, pixel_dims: Some(PixelDimensions { xppu: 2835, yppu: 2835, unit: Meter }), palette: None, gama_chunk: Some(ScaledFloat(45455)), chrm_chunk: None, frame_control: None, animation_control: None, compression: Fast, source_gamma: Some(ScaledFloat(45455)), source_chromaticities: Some(SourceChromaticities { white: (ScaledFloat(31270), ScaledFloat(32900)), red: (ScaledFloat(64000), ScaledFloat(33000)), green: (ScaledFloat(30000), ScaledFloat(60000)), blue: (ScaledFloat(15000), ScaledFloat(6000)) }), srgb: Some(Perceptual), icc_profile: None, uncompressed_latin1_text: [], compressed_latin1_text: [], utf8_text: [] }
    println!("{:?}", info);
    let outPutInfo = reader.next_frame(&mut img_data).expect("未能读取frame信息");

    println!("{:?}", outPutInfo); // OutputInfo { width: 310, height: 310, color_type: Rgba, bit_depth: Eight, line_size: 1240 }
    let (data, format) = match outPutInfo.color_type {
        Rgb => (
            {
                println!("00000000000"); //当不引入  use png::ColorType::* 时走这个分支
                img_data
            },
            ClientFormat::U8U8U8,
        ),
        Rgba => (
            {
                println!("11111111111"); //实际走的这个分支
                img_data
            },
            ClientFormat::U8U8U8U8,
        ),
        Grayscale => (
            {
                let mut vec = Vec::with_capacity(img_data.len() * 3);
                for g in img_data {
                    println!("222222222");
                    vec.extend([g, g, g].iter().cloned())
                }
                vec
            },
            ClientFormat::U8U8U8,
        ),
        GrayscaleAlpha => (
            {
                let mut vec = Vec::with_capacity(img_data.len() * 3);
                for ga in img_data.chunks(2) {
                    let g = ga[0];
                    let a = ga[1];
                    vec.extend([g, g, g, a].iter().cloned())
                }
                println!("333333333");
                vec
            },
            ClientFormat::U8U8U8U8,
        ),
        _ => unreachable!("uncovered color type"),
    };

    // let buf=f.bytes();
    // // let mut buf = vec![0; 8];
    // // let n = f.read_buf(&mut buf[..]).unwrap();

    println!("data capacity{:?},len:{:?} ", data.capacity(), data.len());
    let mut cloneData = data.clone();
   
    // cloneData.truncate(1000);
    // cloneData.trunc(1000);
 
}

// 通过前端调用
#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    println!("front.start.Mock.Initializing...");
    std::thread::sleep(std::time::Duration::from_secs(7));
    println!("front.start.Mock.Done initializing.");
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("my_splashscreen") {
        splashscreen.close().unwrap();
    }

    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

// 托盘菜单
pub fn menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let change_ico = CustomMenuItem::new("change_ico".to_string(), "Change Icon");
    let tray_menu = SystemTrayMenu::new()
        .add_submenu(SystemTraySubmenu::new(
            "Language", //
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("lang_english", "English"))
                .add_item(CustomMenuItem::new("lang_zh_CN", "简体中文"))
                .add_item(CustomMenuItem::new("lang_zh_HK", "繁体中文")),
        ))
        .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(change_ico)
        .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(hide)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

// 托盘事件
pub fn tray_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("左键点击");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("右键点击");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("双击");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            // "change_ico" => {
            //     // 更新托盘图标
            //     app.tray_handle()
            //         .set_icon(tauri::Icon::Raw(
            //             include_bytes!("../icons/new-icon.png").to_vec(),
            //         ))
            //         .unwrap(); // TODO 此处存在bug，点击后系统随即异常退出
            // }
            lang if lang.contains("lang_") => {
                Lang::new(
                    app,
                    id,
                    vec![
                        Lang {
                            name: "切换-English",
                            id: "lang_english",
                        },
                        Lang {
                            name: "切换-繁体中文",
                            id: "lang_zh_HK",
                        },
                        Lang {
                            name: "切换-简体中文",
                            id: "lang_zh_CN",
                        },
                    ],
                );
            }
            "hide" => {
                println!("点击隐藏");
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "show" => {
                println!("点击显示");
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            "quit" => {
                println!("点击退出");
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {
            println!("其它操作");
        }
    }
}

struct Lang<'a> {
    name: &'a str,
    id: &'a str,
}

impl Lang<'static> {
    fn new(app: &AppHandle, id: String, langs: Vec<Lang>) {
        langs.iter().for_each(|lan| {
            let handle = app.tray_handle().get_item(lan.id);
            if lan.id == id {
                // 设置菜单名称
                handle.set_title(format!(" {}", lan.name)).unwrap();
                handle.set_selected(true).unwrap();
            } else {
                handle.set_title(lan.name).unwrap();
                handle.set_selected(false).unwrap();
            }
        });
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 后端单方（无需前端发起调用) 关闭开屏界面
            let splashscreen_window = app.get_window("my_splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                println!("Mock.Initializing...");
                std::thread::sleep(std::time::Duration::from_secs(3));
                println!("Mock.Done initializing.");
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            args_command,
            init_process,
            read_every_text_file,
            read_img_file,
            close_splashscreen
        ])
        .system_tray(menu())
        .on_system_tray_event(tray_handler)
        // .run(tauri::generate_context!())
        // .expect("error while running tauri application")
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                // dbg!("阻止退出,系统仍在后台运行");
                println!("最后一个窗口界面关闭,系统仍在后台运行");
                api.prevent_exit(); //阻止退出,在后台运行
            }
        });
}
