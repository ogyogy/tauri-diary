// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::prelude::*;
use rusqlite::{Connection, Error, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Message {
    id: i32,
    text: String,
    created_at: i64,
}

fn conn_db() -> Result<Connection, Error> {
    let conn = Connection::open("./test.db")?;
    Ok(conn)
}

fn get_messages(conn: &Connection) -> Result<Vec<Message>> {
    let mut stmt = conn.prepare("SELECT id, text, created_at FROM messages")?;
    let rows = stmt.query_map([], |row| {
        Ok(Message {
            id: row.get(0)?,
            text: row.get(1)?,
            created_at: row.get(2)?,
        })
    })?;

    let mut messages = Vec::new();
    for msg in rows {
        messages.push(msg?);
    }

    Ok(messages)
}

fn insert_message(conn: &Connection, text: String) -> Result<Message> {
    let msg = Message {
        // idは自動的に付与される
        id: 0,
        text: text,
        created_at: Local::now().timestamp(),
    };
    conn.execute(
        "INSERT INTO messages (text, created_at) VALUES (?1, ?2)",
        (&msg.text, &msg.created_at),
    )?;
    Ok(msg)
}

#[tauri::command]
fn get_messages_all() -> Result<Vec<Message>, String> {
    // unwrap()でResultの中身を取得する
    let conn = conn_db().unwrap();
    let msgs = get_messages(&conn).unwrap();
    Ok(msgs)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(text: String) -> Message {
    let conn = conn_db().unwrap();
    let msg = insert_message(&conn, text).unwrap();
    return msg;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_messages_all])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
