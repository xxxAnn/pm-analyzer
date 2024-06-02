// Provides data for a website
// That lets you modify the country files of Victoria 3
mod scanner;

fn main() {
    scanner::scan().unwrap();
}
