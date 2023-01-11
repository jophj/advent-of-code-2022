use std::fs;
use std::io;

fn get_scenic_score(height_line: &Vec<&u8>, height: u8) -> u32 {
    let mut scenic_score = 0;
    for test in height_line {
        if test >= &&height {
            scenic_score += 1;
            break;
        } else {
            scenic_score += 1;
        }
    }
    // println!("scenic score for {} {:?} {scenic_score}", &&height, height_line);
    return scenic_score;
}
fn main() -> io::Result<()> {
    let input_string = fs::read_to_string("./input.txt").unwrap();

    let lines_iterator = input_string.lines().peekable();
    let mut height_matrix = Vec::<Vec<u8>>::new();
    for line in lines_iterator {
        let mut line_vec = Vec::new();
        for height in line.chars() {
            line_vec.push(height.to_string().parse::<u8>().unwrap());
        }
        height_matrix.push(line_vec);
    }
    println!("{:?}", height_matrix);

    let mut result = 0;
    let mut highest_scenic_score = 0;
    let mut y = 1;
    while y < height_matrix.len() - 1 {
        let current_line = &height_matrix[y];
        let mut x = 1;
        while x < current_line.len() - 1 {
            let mut left_visible = false;
            let mut right_visible = false;
            let mut top_visible = false;
            let mut bottom_visible = false;

            let left_height_line = &height_matrix[y][..x].iter().rev().collect::<Vec<&u8>>();
            let right_height_line = &height_matrix[y][(x + 1)..].iter().collect::<Vec<&u8>>();
            let mut top_height_line = Vec::new();
            let mut bottom_height_line = Vec::new();
            for test in height_matrix[..y].iter().rev() {
                top_height_line.push(&test[x]);
            }
            for test in &height_matrix[(y+1)..] {
                bottom_height_line.push(&test[x]);
            }

            let left_scenic_score = get_scenic_score(left_height_line, height_matrix[y][x]);
            let right_scenic_score = get_scenic_score(right_height_line,height_matrix[y][x]);
            let top_scenic_score = get_scenic_score(&top_height_line, height_matrix[y][x]);
            let bottom_scenic_score = get_scenic_score(&bottom_height_line, height_matrix[y][x]);
            let scenic_score = left_scenic_score * top_scenic_score * bottom_scenic_score * right_scenic_score;

            if (left_scenic_score as usize) == left_height_line.len() && left_height_line.last().unwrap() < &&height_matrix[y][x] {
                left_visible = true;
            }
            if (right_scenic_score as usize) == right_height_line.len() && right_height_line.last().unwrap() < &&height_matrix[y][x] {
                right_visible = true;
            }
            if (bottom_scenic_score as usize) == bottom_height_line.len() && bottom_height_line.last().unwrap() < &&height_matrix[y][x] {
                bottom_visible = true;
            }
            if (top_scenic_score as usize) == top_height_line.len() && top_height_line.last().unwrap() < &&height_matrix[y][x] {
                top_visible = true;
            }

            if highest_scenic_score < scenic_score {
                highest_scenic_score = scenic_score;
            }
            result += if left_visible || right_visible || top_visible || bottom_visible { 1 } else { 0 };
            x += 1;
        }
        y += 1;
    }
    let edges = height_matrix.len() * 2 + (height_matrix[0].len() - 2) * 2;
    let total = result + edges;
    println!("{total} {highest_scenic_score}");
    Ok(())
}
