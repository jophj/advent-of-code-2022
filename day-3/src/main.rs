use std::fs;
use std::io;
use std::io::BufRead;

fn find_common_character(arr: &[&str]) -> Option<char> {
    // Get the first string in the array
    let first_string = arr[0];

    // Loop through each character in the first string
    for c in first_string.chars() {
        // Check if this character is present in every other string in the array
        let is_common = arr[1..].iter().all(|s| s.contains(c));

        if is_common {
            return Some(c);
        }
    }

    None
}

fn match_priority(character: char) -> i32 {
    let priority = match character {
        'a'..='z' => character as u8 - 'a' as u8 + 1,
        'A'..='Z' => character as u8 - 'A' as u8 + 27,
        _ => 0,
    } as i32;
    
    priority
}

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut score: i32 = 0;

    // part 1
    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut iter = lines.into_iter();
    for line in iter.clone() {
        let (left, right) = line.split_at(line.len() / 2);

        let duplicate = find_common_character(&[&left, &right]).unwrap();
        let priority = match_priority(duplicate);
        score += priority;

        println!("{}: {}", line, priority);
    }
    println!("{}", score);

    // part 2
    score = 0;
    while let (Some(line1), Some(line2), Some(line3)) = (iter.next(), iter.next(), iter.next()) {
        let badge = find_common_character(&[&line1, &line2, &line3]);
        let priority = match_priority(badge.unwrap());
        score += priority;
    }
    println!("{}", score);
    Ok(())
}
