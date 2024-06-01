use std::io::Write;

pub fn scan() { 
    if std::path::Path::new("scan.cfg").exists() {
        let text = std::fs::read_to_string("scan.cfg").unwrap();

    } else {
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
}