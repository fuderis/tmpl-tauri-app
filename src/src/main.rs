#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]  // WARN: DO NOT REMOVE!!
use app::prelude::*;
use tauri::State;

/// Says hello
#[tauri::command]
async fn hello(name: String, phrase: State<'_, String>) -> StdResult<String, String> {
    let msg = phrase.replace("__NAME__", &name);

    Ok(msg)
}


#[tokio::main]
async fn main() -> Result<()> {
    let phrase = String::from("Hello, __NAME__!");
    
    tauri::Builder::default()
        .manage(phrase)
        .invoke_handler(tauri::generate_handler![
            hello
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}

/// Generates an unique ID
fn _uniq_id() -> String {
    use std::time::{ SystemTime, UNIX_EPOCH };
    
    let millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let random: u16 = rand::random();
    format!("{}{:04x}", millis, random)
}
