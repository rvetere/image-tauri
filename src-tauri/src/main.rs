#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
use serde::Serialize;
use std::{ path::{ Path, PathBuf }, sync::Mutex };
use tauri::{ api::dir::is_dir, State };

use std::fs::{ self, read };
struct DirEntries(Mutex<Vec<PathBuf>>);
// Number of subdirectories in the current entry list.
struct SubDirectoriesCount(Mutex<usize>);

#[derive(Serialize)]
struct BootPayload {
    drives: Vec<String>,
}
#[derive(Debug)]
struct DriveEntries(Mutex<Vec<String>>);
struct ActiveDrive(Mutex<usize>);
struct ActivePath(Mutex<PathBuf>);

#[tauri::command]
fn scan_dir(
    dir_entries: State<DirEntries>,
    active_path: State<ActivePath>,
    sub_dir_count: State<SubDirectoriesCount>
) -> Vec<PathBuf> {
    // Directory scan
    let path = &*active_path.0.lock().unwrap();
    let dir = fs::read_dir(path).expect("❌DIR NOT FOUND");
    let mut entries: Vec<PathBuf> = Vec::new();
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in dir {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    entries.push(entry.path());
                } else {
                    files.push(entry.path());
                }
            }
        }
    }
    *sub_dir_count.0.lock().unwrap() = entries.len() + 1;
    // Sort files and directories, and Append
    entries.sort();
    files.sort();
    entries.append(&mut files);
    // Add parent directory to top
    entries.push(match path.parent() {
        Some(_parent) => _parent.to_path_buf(),
        _ => path.to_path_buf(),
    });
    entries.rotate_right(1);
    // Update state
    *dir_entries.0.lock().unwrap() = entries.clone();
    entries
}

#[tauri::command]
fn count_sub_dir(sub_dir_count: State<SubDirectoriesCount>) -> usize {
    *sub_dir_count.0.lock().unwrap()
}

#[tauri::command]
fn change_drive(
    chg_num: usize,
    active_drive: State<ActiveDrive>,
    drives: State<DriveEntries>,
    active_path: State<ActivePath>
) -> usize {
    let v_drives = drives.0.lock().unwrap().to_vec();
    let new_num = if let Some(_letter) = v_drives.get(chg_num) { chg_num } else { 0 };
    *active_drive.0.lock().unwrap() = new_num;
    *active_path.0.lock().unwrap() = Path::new(dbg!(&v_drives[new_num])).to_path_buf();
    new_num
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn start() -> BootPayload {
    let drives = scan_drive();
    BootPayload { drives }
}

fn main() {
    tauri::Builder
        ::default()
        .manage(DriveEntries(scan_drive().into()))
        .manage(ActiveDrive((0).into()))
        .manage(ActivePath(Default::default()))
        .manage(DirEntries(Default::default()))
        .manage(SubDirectoriesCount(Default::default()))
        .invoke_handler(tauri::generate_handler![start, change_drive, scan_dir, count_sub_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn scan_drive() -> Vec<String> {
    print!("Scanning drives...");
    let mut drives: Vec<String> = Vec::new();
    for a in 0..26 + b'A' {
        let d = format!("{}:\\", a as char);
        if let Ok(_) = is_dir(&d) {
            drives.push(d);
        }
    }
    let unix_dir = "/";
    if let Ok(_) = is_dir(unix_dir) {
        drives.push(unix_dir.to_string());
    }
    print!("drives --> {}", drives.len());
    drives
}

// #![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
// use serde::Serialize;
// use std::{ path::{ Path, PathBuf }, sync::Mutex };
// use tauri::{ api::dir::is_dir, State };
// #[derive(Serialize)]
// struct BootPayload {
//     drives: Vec<String>,
// }

// #[derive(Debug)]
// struct DriveEntries(Mutex<Vec<String>>);
// struct ActiveDrive(Mutex<usize>);
// struct ActivePath(Mutex<PathBuf>);
// #[tauri::command]
// fn change_drive(
//     chg_num: usize,
//     active_drive: State<ActiveDrive>,
//     drives: State<DriveEntries>,
//     active_path: State<ActivePath>
// ) -> usize {
//     let v_drives = drives.0.lock().unwrap().to_vec();
//     let new_num = if let Some(_letter) = v_drives.get(chg_num) { chg_num } else { 0 };
//     *active_drive.0.lock().unwrap() = new_num;
//     *active_path.0.lock().unwrap() = Path::new(dbg!(&v_drives[new_num])).to_path_buf();
//     new_num
// }

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// #[tauri::command]
// fn boot_app() -> BootPayload {
//     let drives = scan_drive();
//     BootPayload { drives }
// }

// fn main() {
//     tauri::Builder
//         ::default()
//         .manage(DriveEntries(scan_drive().into()))
//         .manage(ActiveDrive((0).into()))
//         .manage(ActivePath(Default::default()))
//         .invoke_handler(tauri::generate_handler![greet])
//         .invoke_handler(tauri::generate_handler![boot_app])
//         .invoke_handler(tauri::generate_handler![change_drive])
//         .on_page_load(|window, _payload| {
//             let payload = BootPayload { drives: scan_drive() };
//             window.emit("boot", &Some(payload)).expect("failed to emit event");
//         })
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
// fn scan_drive() -> Vec<String> {
//     print!("Scanning drives...");
//     let mut drives: Vec<String> = Vec::new();
//     for a in 0..26 + b'A' {
//         let d = format!("{}:\\", a as char);
//         if let Ok(_) = is_dir(&d) {
//             drives.push(d);
//         }
//     }
//     let unix_dir = "/";
//     if let Ok(_) = is_dir(unix_dir) {
//         drives.push(unix_dir.to_string());
//     }
//     print!("drives --> {}", drives.len());
//     drives
// }