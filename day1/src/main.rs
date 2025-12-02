use std::fs::File;
use std::io::{self, BufRead, BufReader};

const RANGE_SIZE: i32 = 100;

fn main() -> io::Result<()> {
    let file_path = "./input.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    // first(&mut reader)?;
    second(&mut reader)?;
    Ok(())
}

fn first(reader: &mut BufReader<File>) -> io::Result<()> {
    let mut current_position: i32 = 50;
    let mut answer = 0;
    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        let direction = chars[0];
        let displacement: i32 = line[1..].parse().unwrap();
        match direction {
            'L' => {
                current_position = (current_position - displacement).rem_euclid(RANGE_SIZE);
            }
            'R' => {
                current_position = (current_position + displacement).rem_euclid(RANGE_SIZE);
            }
            _ => continue,
        }
        if current_position == 0 {
            answer += 1;
        }
    }

    println!("answer is this {}", answer);
    Ok(())
}

fn second(reader: &mut BufReader<File>) -> io::Result<()> {
    let mut current_position: i32 = 50;
    let mut answer = 0;
    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        let direction = chars[0];
        let displacement: i32 = line[1..].parse().unwrap();
        let mut delta = 1;
        if direction == 'L' {
            delta = -1;
        }
        let mut i = 0;
        loop {
            if i == displacement {
                break;
            }
            current_position += delta;
            current_position = current_position.rem_euclid(RANGE_SIZE);
            if current_position == 0 {
                answer += 1;
            }
            i += 1;
        }
    }

    println!("answer is this {}", answer);
    Ok(())
}
