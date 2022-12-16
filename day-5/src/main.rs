use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input_string = fs::read_to_string("./input.txt").unwrap();
    let (stack_string, moves_string) = input_string.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    for stack_line in stack_string.lines() {
        if stack_line.starts_with(" 1") {
            break;
        }
        let chars_vector = stack_line.chars().collect::<Vec<char>>();
        let chunks = chars_vector.chunks(4);
        let mut i = 0;
        for chunk in chunks {
            // print!("{:?}", chunk.get(1));
            match chunk.get(1) {
                Some(' ') => print!(""),
                Some(value) => stacks[i].push(*value),
                _ => panic!()
            }
            i += 1;
        }
    }

    print_stacks(&stacks);
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    print_stacks(&stacks);

    for move_string in moves_string.lines() {
        // println!("{}", move_string);
        let mut move_iter = move_string.split_whitespace();
        move_iter.next();
        let mut amount = move_iter.next().unwrap().parse::<usize>().unwrap();
        move_iter.next();
        let from = move_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        move_iter.next();
        let to = move_iter.next().unwrap().parse::<usize>().unwrap() - 1;

        println!("{:?} {:?} {:?}", amount, from, to);
        print_stacks(&stacks);
        let mut moved_crates = Vec::new();
        while amount > 0 {
            let crate_name = stacks[from].pop();
            moved_crates.push(crate_name.unwrap_or_default());
            // stacks[to].push(crate_name.unwrap_or_default()); // part 1
            amount -= 1;
        }
        moved_crates.reverse();

        // part 2
        for crate_to_move in  moved_crates{
            stacks[to].push(crate_to_move); 
        }
    }
    print_stacks(&stacks);
    Ok(())
}

fn print_stacks(stacks: &[Vec<char>]) {
    println!("STACKS");
    for stack in stacks.iter() {
        println!("{:?}", stack);
    }
    println!();
}
