use execute::generic_array::arr;
use tauri::{AppHandle, Event, Manager};
use core::{num, time};
use std::{usize, vec};
use chrono::{Duration, FixedOffset};
use serde_json::{Value};
use tokio::time::sleep;

use rand::Rng;

use crate::{http_requests::*};
use crate::{format_data::*};
use crate::{payload::*};

// Time in milliseconds, has to be the same as the animation speed of the chart
const UPDATE_PERIODE: usize = 300;

const NUM_STATEMENTS_SIZE: usize = 10;
static mut NUM_STATEMENTS: Vec<usize> = Vec::new();
static mut LOG_STATEMENTS: Vec<String> = Vec::new();
static mut FILTERED_STATEMENTS_COUNTER: Vec<usize> = Vec::new();

static mut FILTERS: Vec<Filter> = Vec::new();

use crate::{IP_ADDRESS, PASSWORD, PORT, USERNAME, UPDATER_STARTED};


#[tauri::command]
pub fn send_existing_data_to_log() -> Vec<String> {
    println!("send_existing_data_to_frontend called!");
    unsafe { 
        LOG_STATEMENTS.clone()   
    }
}

#[tauri::command]
pub fn send_existing_data_to_filter() -> Vec<usize> {
    println!("send_existing_data_to_frontend called!");
    unsafe { 
         FILTERED_STATEMENTS_COUNTER.clone()
    }
}

#[tauri::command]
pub fn update_filters(id_: String, filter_rules: Vec<FilterRule>) {
    println!("update_filters called!");
    unsafe {
        let mut is_new_filter = true;
        for filter in FILTERS.iter_mut() {
            if (filter.id == id_) {
                filter.rules = filter_rules.clone();
                is_new_filter = false;
            }
        }
        if (is_new_filter) {
            FILTERS.push( Filter { id: id_, rules: filter_rules.clone(), statement_counter: 0 });
            println!("FILTERS length (update): {}", FILTERS.len());
        }
    }
}

#[tauri::command]
pub fn deregister_filter(filter_id: String) {
    println!("remove id: {}", filter_id);
    unsafe {
        println!("FILTERS LENGTH. {}", FILTERS.len());
        for x in 0.. FILTERS.len() {
            if (FILTERS[x].id == filter_id) {
                FILTERS.remove(x);
                println!("{}. filter removed", x);
                break;
            }
        }
    }
}

// I know this is ugly...
pub fn init_num_statements() {
    for _i in 0..NUM_STATEMENTS_SIZE {
        unsafe { NUM_STATEMENTS.push(0); }
    }
}

pub async fn connect_to_lrs(app_handle: AppHandle, event: Event) {
    if let Some(payload) = event.payload() {
        match serde_json::from_str::<ConnectionParamsPayload>(payload) {
            Ok(connection_payload) => {
                unsafe {
                    IP_ADDRESS = connection_payload.ip_address;
                    PORT = connection_payload.port;
                    USERNAME = connection_payload.username;
                    PASSWORD = connection_payload.password;
                    let (connected_to_lrs_successfully, response) = test_lrs_connection(app_handle.clone()).await;
                    let _ = app_handle.emit_all("connection-response", ConnectionResponsePayload { successfull: connected_to_lrs_successfully, response: response.clone() });
                    println!("Connected: {}", connected_to_lrs_successfully);
                    if connected_to_lrs_successfully {
                        if !UPDATER_STARTED {
                            tokio::spawn(async move {
                                update_frontend(app_handle).await;
                            });
                            UPDATER_STARTED = true;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to deserialize payload: {:?}", e);
            }
        }
    } else {
        eprintln!("No payload found in the event.");
    }
}

fn send_filtered_statements_counter(app_handle: AppHandle) {
    unsafe {
        for filter in FILTERS.clone() {
            app_handle.emit_all((filter.id + "-filtered-statements-counter").as_str(), PayloadUsize { value: filter.statement_counter });
        }
    }
}

pub async fn update_frontend(app_handle: AppHandle) {
    let mut connectionEstablished = true;
    let date_time = chrono::offset::Utc::now();
    let offset = FixedOffset::east_opt(-2 * 3600).expect("Could not get offset!"); // east_opt returns an Option<FixedOffset>

    // Apply the offset to the current UTC time
    let local_time = date_time.with_timezone(&offset);
    let mut timestamp = format!("{}", local_time.format("%Y-%m-%dT%H:%M:%SZ"));//"2023-06-20T08:16:47.696000000Z".to_string();
    loop {
        match request_statements_since(&timestamp).await {
            Ok(response) => {
                if !connectionEstablished {
                    connectionEstablished = true;
                    let _ = app_handle.emit_all("connection-response", ConnectionResponsePayload { successfull: true, response: response.clone() });
                }
                let v: Value = serde_json::from_str(&response).expect("Error on deserialization of json");
                if let Some(statements) = v["statements"].as_array() {
                    let mut new_num_statements = 0;
                    if statements.is_empty() {
                    } else {
                        unsafe {
                            new_num_statements = get_num_statements(v["statements"].clone());
                            let log_statements = get_condensed_statements(v["statements"].clone());
                            let prettified_statements = get_prettified_statements(v["statements"].clone());
                            LOG_STATEMENTS.append(&mut log_statements.clone());
                            let _ = app_handle.emit_all("log-statements-event", StatementLogPayload {densed_statements: log_statements, full_statements: prettified_statements });
                            FILTERS = get_filtered_statements_counters(v["statements"].clone(), FILTERS.clone());
                            send_filtered_statements_counter(app_handle.clone());
                        }
                        timestamp = statements[0]["stored"].to_string().replace('"', "");
                    }
                    unsafe {
                        let mut rng = rand::thread_rng();
                        //let _ = app_handle.emit_all("prog-num-statements", PayloadUsize { value: rng.gen_range(0..10) });
                        let _ = app_handle.emit_all("prog-num-statements", PayloadUsize { value: new_num_statements });
                    }
                } else {
                    println!("Something wrong: {}", response);
                }
            },
            Err(e) => {
                if connectionEstablished {
                    connectionEstablished = false;
                    let _ = app_handle.emit_all("connection-response", ConnectionResponsePayload { successfull: false, response: e.to_string() });
                }
                eprintln!("Error: {}", e)
            },
        }
        sleep(time::Duration::from_millis(UPDATE_PERIODE.try_into().unwrap())).await;
    }
}