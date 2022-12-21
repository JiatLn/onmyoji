use crate::Account;
use crate::MyState;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn get_account(state: tauri::State<MyState>) -> Account {
    let acc = state.0.lock().unwrap().clone();
    acc
}

#[tauri::command]
pub fn consum_currency(state: tauri::State<MyState>, ty: &str, amount: u32) -> Result<(), String> {
    let mut acc = state.0.lock().unwrap();
    acc.currency.consum(ty, amount).map_err(|e| e.to_string())?;
    acc.save().map_err(|e| e.to_string())
}
