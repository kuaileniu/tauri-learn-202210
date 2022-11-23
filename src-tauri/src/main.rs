#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::process::Command;
use std::{thread, time};
use tauri::SystemTrayEvent;
use tauri::{AboutMetadata, Manager, WindowMenuEvent};
use tauri::{AppHandle, Menu, MenuItem, Submenu};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};
use tauri::{SystemTray, Window};
use test5::image_encode_base64;

const MENU_ID_NEW_WINDOW: &str = "new_window";

const MENU_ID_HIDDEN_NEW_WINDOW: &str = "hidden_new_window";
const MENU_ID_CLOSE_NEW_WINDOW: &str = "close_new_window";

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
        thread::sleep(time::Duration::from_secs(1));
    });
}

#[tauri::command]
fn read_every_text_file(path: std::path::PathBuf) -> String {
    std::fs::read_to_string(path).unwrap()
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

#[tauri::command]
fn run_elf(path: &str) -> String {
    println!("rust run_elf path ==== {:?}", path);
    let mut local_command = Command::new(path);
    let result = local_command.output();
    match result {
        Ok(v) => format!("执行成功 {:?}", v),
        Err(e) => format!("执行程序失败 {:?}", e),
    }
}

#[tauri::command]
fn read_img_file(path: &str) -> String {
    image_encode_base64(path)
}

// 托盘菜单
pub fn tray_menu() -> SystemTray {
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

pub fn menu() -> Menu {
    let app_name = "test5";
    let mut menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            app_name,
            Menu::new()
                .add_native_item(MenuItem::About(
                    app_name.to_string(),
                    AboutMetadata::default(),
                ))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Services)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ));
    }

    let mut file_menu = Menu::new();
    file_menu = file_menu.add_native_item(MenuItem::CloseWindow);
    #[cfg(not(target_os = "macos"))]
    {
        file_menu = file_menu.add_native_item(MenuItem::Quit);
    }

    file_menu = file_menu.add_item(CustomMenuItem::new(MENU_ID_NEW_WINDOW, "新建模态窗口"));
    file_menu = file_menu.add_item(CustomMenuItem::new(
        MENU_ID_HIDDEN_NEW_WINDOW,
        "隐藏模态窗口",
    ));
    file_menu = file_menu.add_item(CustomMenuItem::new(
        MENU_ID_CLOSE_NEW_WINDOW,
        "关闭模态窗口",
    ));
    menu = menu.add_submenu(Submenu::new("File", file_menu));

    #[cfg(not(target_os = "linux"))]
    let mut edit_menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::Undo);
        edit_menu = edit_menu.add_native_item(MenuItem::Redo);
        edit_menu = edit_menu.add_native_item(MenuItem::Separator);
    }
    #[cfg(not(target_os = "linux"))]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::Cut);
        edit_menu = edit_menu.add_native_item(MenuItem::Copy);
        edit_menu = edit_menu.add_native_item(MenuItem::Paste);
    }
    #[cfg(target_os = "macos")]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::SelectAll);
    }
    #[cfg(not(target_os = "linux"))]
    {
        menu = menu.add_submenu(Submenu::new("Edit", edit_menu));
    }
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            "View",
            Menu::new().add_native_item(MenuItem::EnterFullScreen),
        ));
    }

    let mut window_menu = Menu::new();
    window_menu = window_menu.add_native_item(MenuItem::Minimize);
    #[cfg(target_os = "macos")]
    {
        window_menu = window_menu.add_native_item(MenuItem::Zoom);
        window_menu = window_menu.add_native_item(MenuItem::Separator);
    }
    window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
    menu = menu.add_submenu(Submenu::new("Window", window_menu));

    menu
}

pub fn menu_event(event: WindowMenuEvent) {
    println!("menu_event-------");
    let menu_id = event.menu_item_id();
    println!("menu_id: {:?}", menu_id);
    match event.menu_item_id() {
        MENU_ID_NEW_WINDOW => {
            if let Some(baidu_window) = event.window().get_window("baidu_window") {
                baidu_window.show().unwrap();
                println!("重新显示已有窗口");
            } else {
                let baidu_window = tauri::WindowBuilder::new(
                    event.window(),
                    "baidu_window",
                    // tauri::WindowUrl::External("https://www.baidu.com/".parse().unwrap()),
                    tauri::WindowUrl::App("page/new_window.html".into()),
                );
                baidu_window
                    .center()
                    // .always_on_top(true)
                    .decorations(false)
                    .inner_size(800.0, 400.0)
                    .build()
                    .unwrap();
                println!("新建窗口");
            }
            event
                .window()
                .get_window("main")
                .unwrap()
                .set_ignore_cursor_events(true); //set_ignore_cursor_events 的开关可做出模态窗口效果 //.open_devtools();//open_devtools不存在于发布版本的api中
        }

        MENU_ID_CLOSE_NEW_WINDOW => {
            if let Some(main_window) = event.window().get_window("main") {
                main_window.set_ignore_cursor_events(false); //set_ignore_cursor_events 的开关可做出模态窗口效果
            }
            if let Some(baidu_window) = event.window().get_window("baidu_window") {
                baidu_window.close().unwrap();
                println!("成功关闭窗口");
            } else {
                println!("未能成功关闭窗口");
            }
        }

        MENU_ID_HIDDEN_NEW_WINDOW => {
            if let Some(main_window) = event.window().get_window("main") {
                main_window.set_ignore_cursor_events(false);
            }
            if let Some(baidu_window) = event.window().get_window("baidu_window") {
                baidu_window.hide().unwrap();
                println!("成功隐藏窗口");
            } else {
                println!("未能成功隐藏窗口");
            }
        }

        _ => {}
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
            // let docs_window = tauri::WindowBuilder::new(
            //     app,
            //     "external", /* the unique window label */
            //     tauri::WindowUrl::External("https://tauri.app/".parse().unwrap())
            //   ).build()?;

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
            close_splashscreen,
            run_elf,
            read_img_file
        ])
        .menu(menu())
        .on_menu_event(menu_event)
        .system_tray(tray_menu())
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
