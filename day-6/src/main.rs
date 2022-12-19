use std::collections::HashSet;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input_string = fs::read_to_string("./input.txt").unwrap();
    
    let mut index = 14; // part 1 requires 4 where you see 14
    while index < input_string.len() {
        let window = &input_string[index - 14..index];
        
        let set: HashSet<char> = HashSet::from_iter(window.chars());
        if set.len() == 14 {
            break;
        }
        index += 1;
    }
    println!("{}", index);

    Ok(())
}
