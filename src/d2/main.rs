use std::fs::read;

fn main() {
    let fileContent = read("./src/input.txt");
    let binding = fileContent.unwrap();
    let fileTxt = String::from_utf8_lossy(&binding);
    println!("{}", fileTxt);
}