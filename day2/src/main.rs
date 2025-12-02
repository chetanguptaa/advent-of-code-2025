use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "./input.txt";
    let mut content = String::from("");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }
        content.push_str(&line);
    }
    let _ = first(&content);
    let _ = second(&content);
    Ok(())
}

fn first(content: &str) -> io::Result<()> {
    let mut ans = 0;
    let ranges: Vec<&str> = content.split(',').collect();
    let mut i = 0;
    while i < ranges.len() {
        let range: Vec<&str> = ranges[i].split('-').collect();
        let range1: i64 = range[0].parse().unwrap();
        let range2: i64 = range[1].parse().unwrap();
        let mut j = range1;
        while j <= range2 {
            let j_str = j.to_string();
            if j_str.starts_with('0') {
                ans += j;
            }
            if j_str.len() % 2 == 0 {
                let first_half = &j_str[0..j_str.len() / 2];
                let second_half = &j_str[j_str.len() / 2..];
                if first_half == second_half {
                    ans += j;
                }
            }
            j += 1;
        }
        i += 1;
    }
    println!("answer is this {}", ans);
    Ok(())
}

fn all_chunks_equal(s: &str, chunks: usize) -> bool {
    let len = s.len();
    if len % chunks != 0 {
        return false;
    }
    let chunk_len = len / chunks;
    let first = &s[0..chunk_len];
    s.as_bytes()
        .chunks(chunk_len)
        .all(|c| c == first.as_bytes())
}

fn is_invalid(id: i64) -> bool {
    let s = id.to_string();
    if s.starts_with('0') {
        return true;
    }
    for &n in &[2, 3, 5, 7] {
        if all_chunks_equal(&s, n) {
            return true;
        }
    }
    false
}

fn second(content: &str) -> io::Result<()> {
    let mut ans: i64 = 0;
    for range_str in content.split(',') {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        for id in start..=end {
            if is_invalid(id) {
                ans += id;
            }
        }
    }
    println!("answer is this {}", ans);
    Ok(())
}
