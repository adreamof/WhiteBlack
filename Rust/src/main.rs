// 阻止在Windows下打开控制台工具
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use whiteblack::api::send_email;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test, hello_world, send_email_click])
        .run(tauri::generate_context!())
        .expect("Tauri程序运行时错误!");

}

#[tauri::command]
fn test() -> &'static str {
    println!("Hello World!");
    return "Hello World!";
}

#[tauri::command]
fn hello_world() {
    println!("Hello World! Success to open Tauri!");
}

#[tauri::command]
fn send_email_click() {
    send_email();
}