use std::fs;
use std::io;
use std::io::BufRead;

fn generate_sections_vec(section_pattern: Option<&str>) -> Vec<i32> {
    let mut asd = section_pattern.unwrap().split('-');
    // let start = asd.next().unwrap().parse::<i32>().unwrap();
    // let end = asd.next().unwrap().parse::<i32>().unwrap();
    let start = asd.next().unwrap().parse::<i32>().unwrap();
    let end = asd.next().unwrap().parse::<i32>().unwrap();
    let mut sections = Vec::new();
    for c in start..=end {
        sections.push(c);
    }
    sections
}

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut score: i32 = 0; // part 1
    let mut second_score: i32 = 0; // part 2

    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>().unwrap();
    let iter = lines.into_iter();
    for line in iter.clone() {
        let mut sections_string = line.split(',');
        let first_sections = generate_sections_vec(sections_string.next());
        let second_sections = generate_sections_vec(sections_string.next());
        
        if first_sections.iter().all(|f| second_sections.contains(f)) || second_sections.iter().all(|f| first_sections.contains(f)) {
            score += 1;
        }
        if first_sections.iter().any(|f| second_sections.contains(f)) || second_sections.iter().any(|f| first_sections.contains(f)) {
            second_score += 1;
        }
        
        // println!("{} {} {}", first_sections, second_sections, contained);
    }
    println!("{}", score);
    println!("{}", second_score);
    Ok(())
}

