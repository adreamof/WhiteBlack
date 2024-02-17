// 阻止在Windows下打开控制台工具
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test, hello_world])
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