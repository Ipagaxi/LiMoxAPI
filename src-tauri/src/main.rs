// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

use http_requests::test_lrs_connection;
use tauri::Manager;

//use lrs::start_lrs;
pub mod format_data;
pub mod http_requests;
pub mod send_data_to_frontend;
pub mod payload;
mod lrs;

use crate::{send_data_to_frontend::*};
use crate::{payload::*};

static mut IP_ADDRESS: String = String::new();
static mut PORT: String = String::new();
static mut USERNAME: String = String::new();
static mut PASSWORD: String = String::new();
static mut UPDATER_STARTED: bool = false;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.app_handle();
            let id = app.listen_global("connect-lrs", move |event| {
                tauri::async_runtime::spawn(connect_to_lrs(app_handle.clone(), event));
            });
            let log = app.listen_global("log", |event| {
                println!("Frontend: {:?}", event.payload());
              });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_data_to_frontend::send_existing_data_to_log,
            send_data_to_frontend::send_existing_data_to_filter,
            send_data_to_frontend::update_filters,
            send_data_to_frontend::deregister_filter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}