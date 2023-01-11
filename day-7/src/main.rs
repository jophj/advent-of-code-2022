use std::fs;
use std::io;

struct FolderData {
    path: String,
    size: i32,
}
enum Command {
    Cd(String),
    Ls,
    None
}
// impl Command {
//     fn to_string(&self) -> String {
//         match self {
//             Command::Cd(value) => ["cd", value].join(" ").to_string(),
//             Command::Ls => "ls".to_owned(),
//             Command::None => "".to_owned(),
//         }
//     }
// }

fn parse_command (line: &str) -> Command {
    if line.contains("cd") {
        let destination = &line[5..line.len()];
        return Command::Cd(destination.to_owned());
    }
    if line.contains("ls") {
        return Command::Ls;
    }
    Command::None
}
fn main() -> io::Result<()> {
    let input_string = fs::read_to_string("./input.txt").unwrap();

    let mut lines_iterator = input_string.lines().peekable();
    let mut cwd = String::new();
    let mut directories: Vec<FolderData> = Vec::new();
    while lines_iterator.peek().is_some() {
        let line = lines_iterator.next();
        let command = parse_command(line.unwrap());
        // println!("{:?}", command.to_string());
        
        println!("CWD {}", cwd);
        match command {
            Command::Cd(value) => {
                match value.as_str() {
                    "/" => cwd.push_str("/"),
                    ".." => {
                        let last_index = cwd[0..cwd.len() - 1].rfind("/");
                        cwd = cwd[0..last_index.unwrap() + 1].to_string();
                    }
                    d => cwd.push_str([&d ,"/"].join("").as_str())
                }
            },
            Command::Ls => {
                let mut size = 0;
                while lines_iterator.peek().is_some() && !lines_iterator.peek().unwrap().starts_with("$") {
                    let line = lines_iterator.next().unwrap();
                    if !line.starts_with("dir") {
                        let (size_string, _) = line.split_once(" ").unwrap();
                        size += size_string.parse::<i32>().unwrap();

                    }
                    println!("{}", line);
                }
                directories.push(FolderData { path: cwd.clone(), size });
            },
            Command::None => {},
        }
    }

    let mut result = 0; // part 1 result
    let mut directories_totals = Vec::new();
    for dir in &directories {
        let total_size = directories.iter().map(|item| {
            if !item.path.starts_with(&dir.path) {
                return 0
            }
            item.size
        }).reduce(|sum, size| sum + size);


        if total_size.unwrap() < 100000 {
            result += total_size.unwrap();
        }

        directories_totals.push(FolderData { path: dir.path.to_string(), size: total_size.unwrap() });
        println!("{} {}", dir.path, total_size.unwrap());
    }
    println!("{}", result);
    
    // part 2
    let total_space = 70000000;
    let needed_space = 30000000;
    let used_space = directories_totals.get(0).unwrap().size; 
    let available_space = total_space - used_space;
    let to_remove = directories_totals.iter().reduce(|smallest, item| {
        if available_space + item.size > needed_space {
            return item;
        }
        return smallest;
    });
    println!("{} {}", to_remove.unwrap().path, to_remove.unwrap().size); // part 2 result
    Ok(())
}
