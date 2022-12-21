#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::core::{tauri_command::*, Account};
use std::sync::Mutex;

pub mod core;

pub struct MyState(Mutex<Account>);

fn main() {
    let account = Account::load().expect("account not found");

    tauri::Builder::default()
        .manage(MyState(Mutex::new(account)))
        .invoke_handler(tauri::generate_handler![get_account, consum_currency])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
