use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "./input.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    // let _ = first(&mut reader);
    let _ = second(&mut reader);
    Ok(())
}

fn first(reader: &mut BufReader<File>) -> io::Result<()> {
    let mut ans = 0;
    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        let mut biggest = 1;
        let mut i = 0;
        loop {
            if i == line.len() - 1 {
                break;
            }
            let mut j = i + 1;
            let mut n1 = String::from(chars[i]);
            loop {
                if j == line.len() {
                    break;
                }
                if j != i + 1 {
                    n1.pop();
                }
                let n2 = String::from(chars[j]);
                n1.push_str(&n2);
                let num = n1.parse().unwrap();
                if num > biggest {
                    biggest = num;
                }
                j += 1;
            }
            i += 1;
        }
        ans += biggest;
    }
    println!("answer is this {}", ans);
    Ok(())
}

fn second(reader: &mut BufReader<File>) -> io::Result<()> {
    let mut ans: usize = 0;
    for line in reader.lines() {
        let line = line?;
        let mut biggest = 1;
        let mut i = 0;
        const MAX_NO_OF_BATTERIES: usize = 12;
        loop {
            if i == line.len() - 1 {
                break;
            }
            let mut stack: Vec<char> = Vec::new();
            let mut to_remove = line.len().saturating_sub(MAX_NO_OF_BATTERIES);
            for c in line.chars() {
                while to_remove > 0 && !stack.is_empty() && stack.last().unwrap() < &c {
                    stack.pop();
                    to_remove -= 1;
                }
                stack.push(c);
            }
            let a: String = stack.into_iter().take(MAX_NO_OF_BATTERIES).collect();
            let temp: usize = a.parse().unwrap();
            if temp > biggest {
                biggest = temp;
            }
            i += 1;
        }
        ans += biggest;
    }
    println!("answer is this {}", ans);
    Ok(())
}
