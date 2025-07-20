use arboard::Clipboard;
use serde::{ Deserialize, Serialize };
use std::fs::{ remove_file, File, OpenOptions };
use std::io::{ BufReader, BufWriter };
use std::thread::{ sleep, spawn };
use std::time::Duration;
use tauri::ipc::Channel;

#[derive(Serialize, Deserialize)]
struct ClipBoardHistory {
    items: Vec<String>,
}

const PATH: &str = "./clip-history.json";

#[tauri::command]
fn wipe_all() {
    let _ = remove_file(PATH);
}

#[tauri::command]
fn copy(data: String) -> Result<(), String> {
    // let mut clipboard = Clipboard::new().unwrap();
    // clipboard.set_text(data).unwrap();

    let mut clipboard = Clipboard::new().map_err(|e|
        format!("Failed to access clipboard: {:?}", e)
    )?;

    clipboard.set_text(data).map_err(|e| format!("Failed to set clipboard text: {:?}", e))
}

#[tauri::command]
fn load_last_n_entries(n: usize) -> Vec<String> {
    if let Ok(history) = load_history() {
        history.items.into_iter().rev().take(n).collect()
    } else {
        vec![]
    }
}

#[tauri::command]
fn init(on_event: Channel<String>) {
    spawn(move || {
        // let mut clipboard = Clipboard::new().unwrap();

        let mut clipboard = match Clipboard::new() {
            Ok(cb) => cb,
            Err(e) => {
                eprintln!("Failed to access clipboard: {:?}", e);
                return; // Exit thread early
            }
        };

        loop {
            if let Ok(data) = clipboard.get_text() {
                let mut history = load_history().unwrap_or_else(|_| ClipBoardHistory {
                    items: vec![],
                });

                if
                    history.items
                        .last()
                        .map(|last| last != &data)
                        .unwrap_or(true)
                {
                    history.items.push(data.clone());
                    save_history(&history).unwrap();

                    on_event.send(data).unwrap();
                }
            }
            sleep(Duration::from_secs(2));
        }
    });
}

fn load_history() -> Result<ClipBoardHistory, std::io::Error> {
    let file: File = File::open(PATH)?;
    let reader = BufReader::new(file);
    let history = serde_json::from_reader(reader)?;
    Ok(history)
}

fn save_history(history: &ClipBoardHistory) -> Result<(), std::io::Error> {
    let file = OpenOptions::new().create(true).write(true).truncate(true).open(PATH)?;

    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, history)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![init, load_last_n_entries, wipe_all, copy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
