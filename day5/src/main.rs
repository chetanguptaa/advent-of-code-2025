use std::cmp;
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
    let mut found_break = false;
    let mut vec: Vec<(usize, usize)> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim().is_empty() && !found_break {
            found_break = true;
            continue;
        }
        if !found_break {
            let ranges: Vec<&str> = line.split('-').collect();
            let range1: usize = ranges[0].parse().unwrap();
            let range2: usize = ranges[1].parse().unwrap();
            vec.push((range1, range2));
        } else {
            let id: usize = line.parse().unwrap();
            for v in vec.iter() {
                if id >= v.0 && id <= v.1 {
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("answer is this {}", ans);
    Ok(())
}

fn second(reader: &mut BufReader<File>) -> io::Result<()> {
    let mut vec: Vec<(usize, usize)> = Vec::new();
    let mut ans: usize = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }
        let ranges: Vec<&str> = line.split('-').collect();
        let range1: usize = ranges[0].parse().unwrap();
        let range2: usize = ranges[1].parse().unwrap();
        vec.push((range1, range2));
    }
    vec.sort();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    let mut cur_start = vec[0].0;
    let mut cur_end = vec[0].1;
    for i in 1..vec.len() {
        let (start, end) = vec[i];
        if start <= cur_end + 1 {
            cur_end = cmp::max(cur_end, end);
        } else {
            merged.push((cur_start, cur_end));
            [cur_start, cur_end] = [start, end];
        }
    }
    merged.push((cur_start, cur_end));
    for i in 0..merged.len() {
        let (s, e) = merged[i];
        ans += e - s + 1;
    }
    println!("answer is this {}", ans);
    Ok(())
}
