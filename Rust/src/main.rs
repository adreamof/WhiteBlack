// 阻止在Windows下打开控制台工具
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use whiteblack::{api::get_my_task_by_supplier, entity::RequestParams};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test, hello_world, get_task])
        .run(tauri::generate_context!())
        .expect("Tauri程序运行时错误!");

}

#[tauri::command]
fn get_task() -> Result<String, String> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(get_my_task_by_supplier(RequestParams::new())).map_err(|e| e.to_string())
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