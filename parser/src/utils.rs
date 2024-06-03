use super::{node::Node, Tree};
use std::{ffi::OsString, fs, io::Write, path::PathBuf};
use crate::Parser;

pub fn get_scan_paths() -> (String, String) {
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
        file.write_all(format!("{}\n{}", path.trim(), game_path.trim()).as_bytes()).unwrap();
    }

    let text: String = std::fs::read_to_string("scan.cfg").unwrap();
    let mut lines = text.lines();
    let mod_path = lines.next().unwrap();
    let game_path = lines.next().unwrap();

    (mod_path.to_string(), game_path.to_string())
}


pub fn stringify_tree(node: &Node, depth: usize) -> String {
    let mut result = String::new();
    for _ in 0..depth {
        result.push_str("  ");
    }
    result.push_str(&format!("{}\n", node.name()));
    for child in &node.children() {
        result.push_str(&stringify_tree(child, depth + 1));
    }

    result
}

pub fn merge_trees(trees: &[Tree]) -> Tree {
    if trees.len() == 0 {
        return Tree::new(Node::new("empty".to_owned(), 0), 1);
    }
    let mut new_tree = trees[0].clone();
    for tree in trees.iter().skip(1) {
        new_tree = new_tree.merge(tree);
    }
    new_tree
}

pub fn generate_tree(paths: Vec<String>) -> Tree {
    let trees = generate_trees(paths);
    merge_trees(&trees)
}

pub fn generate_trees_and_remember_files(paths: Vec<String>) -> (Vec<(String, Tree)>) {
    let mut trees = Vec::new();
    for path in paths {
        //println!("Parsing file: {}", path);
        let tree = parse_file(&path);
        trees.push((path, tree));
    }
    trees
}

pub fn generate_trees(paths: Vec<String>) -> Vec<Tree> {
    let mut trees = Vec::new();
    for path in paths {
        //println!("Parsing file: {}", path);
        let tree = parse_file(&path);
        trees.push(tree);
    }
    trees
}

pub fn parse_file(path: &str) -> Tree {
    let text = std::fs::read_to_string(path).unwrap();
    let parser = Parser::new();
    parser.parse(text)
}

pub fn get_paths(mod_path: &str, game_path: &str, extension: &str) -> Vec<String> {
    let mut mod_path_modified = mod_path.to_string();
    mod_path_modified.push_str(extension);
    let mut game_path_modified = game_path.to_string();
    game_path_modified.push_str(extension);

    discriminate(mod_path_modified, game_path_modified)
}

pub fn discriminate(mod_path: String, game_path: String) -> Vec<String> {
    // Read the mod files
    let mod_files = match fs::read_dir(&mod_path) {
        Ok(files) => files,
        Err(_) => return Vec::new(),
    };

    // Read the game files
    let game_files_raw = match fs::read_dir(&game_path) {
        Ok(files) => files,
        Err(_) => return Vec::new(),
    };

    // Collect mod files as PathBuf
    let mod_file_entries: Vec<PathBuf> = mod_files
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    return Some(entry.path());
                }
            }
            None
        })
        .collect();

    // Collect mod file names for comparison
    let mod_file_names: Vec<OsString> = mod_file_entries.iter()
        .map(|path| path.file_name().unwrap().to_os_string())
        .collect();

    // Collect game files that are not in the mod files
    let game_file_entries: Vec<PathBuf> = game_files_raw
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    if !mod_file_names.contains(&entry.file_name()) {
                        return Some(entry.path());
                    }
                }
            }
            None
        })
        .collect();

    // Combine mod files with unique game files
    let mut all_files = mod_file_entries;
    all_files.extend(game_file_entries);

    //Check if the files are .txt
    all_files.retain(|path| path.extension().map_or(false, |ext| ext == "txt"));

    // Convert to Vec<String>
    all_files
        .iter()
        .filter_map(|path| path.to_str().map(|s| s.to_string()))
        .collect()
}