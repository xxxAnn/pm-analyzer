use std::{ffi::OsString, fs::DirEntry, io::Write};

pub fn scan() { 
    if !std::path::Path::new("scan.cfg").exists() {
        // Ask for the file
        println!("Please provide the path to the mod folder you want to scan:");
        let mut path = String::new();
        std::io::stdin().read_line(&mut path).unwrap();
        println!("Please provide the path to the game folder:");
        let mut game_path = String::new();
        std::io::stdin().read_line(&mut game_path).unwrap();

        // Write the file
        let mut file: std::fs::File = std::fs::File::create("scan.cfg").unwrap();
        file.write(format!("{}\n{}", path.trim(), game_path.trim()).as_bytes()).unwrap();
    }

    let text: String = std::fs::read_to_string("scan.cfg").unwrap();
    let mut lines = text.lines();
    let mod_path = lines.next().unwrap();
    let game_path = lines.next().unwrap();
}

fn discriminate(mod_path: String, game_path: String) {
    // Read the mod and game files
    // Prioritize the mod files
    let mod_files = std::fs::read_dir(mod_path).unwrap();
    let game_files_raw = std::fs::read_dir(game_path).unwrap();
    let mut files: Vec<DirEntry> = mod_files.filter(|entry| entry.as_ref().unwrap().file_type().unwrap().is_file()).map(|entry| entry.unwrap()).collect();
    let mut game_files = game_files_raw.filter(|entry| entry.as_ref().unwrap().file_type().unwrap().is_file()).filter(
        |entry| files.iter().map(|e: &std::fs::DirEntry| e.file_name()).collect::<Vec<OsString>>().contains(&entry.as_ref().unwrap().file_name())
    ).map(|entry| entry.unwrap());
}