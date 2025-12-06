use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "./input.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let re = Regex::new(r"\s+").unwrap();
    // let _ = first(&mut reader, &re);
    let _ = second(&mut reader);
    Ok(())
}

fn first(reader: &mut BufReader<File>, re: &Regex) -> io::Result<()> {
    let mut num_vec: Vec<(Vec<usize>, String)> = Vec::new();
    let mut ans = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }
        let new_line = re.replace_all(&line, " ").trim().to_string();
        let first_char = new_line.chars().next();
        if let Some(c) = first_char {
            if c == '*' || c == '+' {
                let operators: Vec<&str> = new_line.split(' ').collect();
                for (i, o) in operators.iter().enumerate() {
                    if let Some((_, op_ref)) = num_vec.get_mut(i) {
                        *op_ref = o.to_string();
                    }
                }
                break;
            }
        }
        let parts: Vec<&str> = new_line.split(' ').collect();
        if num_vec.is_empty() {
            num_vec = parts.iter().map(|_| (Vec::new(), String::new())).collect();
        }
        let nums: Vec<usize> = parts
            .iter()
            .filter_map(|p| p.parse::<usize>().ok())
            .collect();
        for (i, num) in nums.iter().enumerate() {
            if let Some((nums_ref, _)) = num_vec.get_mut(i) {
                nums_ref.push(*num);
            }
        }
    }
    for nv in num_vec {
        let numbers = nv.0;
        let operator = nv.1;
        if operator == "*" {
            let mut temp = 1;
            for n in numbers {
                temp *= n;
            }
            ans += temp;
        } else if operator == "+" {
            let mut temp = 0;
            for n in numbers {
                temp += n;
            }
            ans += temp;
        }
    }

    println!("{}", ans);
    Ok(())
}

fn second(reader: &mut BufReader<File>) -> io::Result<()> {
    let mut content = String::new();
    let mut ans: i64 = 0;
    for (idx, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if line.is_empty() {
            continue;
        }
        if idx != 0 {
            content.push_str("\n");
        }
        content.push_str(&line);
    }
    let mut grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let mut max_length_row_length = 0;
    for g in grid.clone() {
        if g.len() > max_length_row_length {
            max_length_row_length = g.len();
        }
    }
    for row in &mut grid {
        if row.len() < max_length_row_length {
            row.extend(std::iter::repeat(' ').take(max_length_row_length - row.len()));
        }
    }
    let grid_length = grid.len();
    if grid_length == 0 {
        println!("0\n0");
        return Ok(());
    }
    let grid_col_length = grid[0].len();
    let mut start_c: usize = 0;
    for cc in 0..=grid_col_length {
        let mut is_blank = true;
        if cc < grid_col_length {
            for r in 0..grid_length {
                if grid[r][cc] != ' ' {
                    is_blank = false;
                    break;
                }
            }
        }
        if is_blank {
            let op = grid[grid_length - 1][start_c];
            let mut score: i64 = if op == '+' { 0 } else { 1 };
            for i in (start_c..cc).rev() {
                let mut n: i64 = 0;
                for j in 0..grid_length - 1 {
                    let ch = grid[j][i];
                    if ch != ' ' {
                        n = n * 10 + ch.to_digit(10).unwrap() as i64;
                    }
                }
                if op == '+' {
                    score += n;
                } else {
                    score *= n;
                }
            }
            ans += score;
            start_c = cc + 1;
        }
    }
    println!("{}", ans);
    Ok(())
}
