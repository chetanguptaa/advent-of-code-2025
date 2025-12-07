use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "./input.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    first(&mut reader)
}

fn first(reader: &mut BufReader<File>) -> io::Result<()> {
    let mut ans: usize = 0;
    first_preprocessing(reader, &mut ans)
}

fn first_preprocessing(reader: &mut BufReader<File>, ans: &mut usize) -> io::Result<()> {
    let mut tachyom_manifold: Vec<Vec<char>> = vec![];
    let mut s_index = 0;
    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }
        let row: Vec<char> = line.chars().collect();
        if i == 0 {
            for (j, char) in row.iter().enumerate() {
                if *char == 'S' {
                    s_index = j;
                }
            }
        }
        tachyom_manifold.push(row);
    }
    let final_grid = first_postprocessing(
        &mut tachyom_manifold,
        1,
        HashSet::from([s_index]),
        HashSet::new(),
        ans,
    );
    println!("final grid {:?}", final_grid);
    println!("final answer {:?}", ans);
    Ok(())
}

fn first_postprocessing<'a>(
    tachyom_manifold: &'a mut Vec<Vec<char>>,
    current_row_index: usize,
    prev_row_pipe_indexes: HashSet<usize>,
    prev_row_caret_indexes: HashSet<usize>,
    ans: &mut usize,
) -> &'a mut Vec<Vec<char>> {
    if current_row_index == tachyom_manifold.len() {
        return tachyom_manifold;
    }
    let mut current_row_pipe_indexes = HashSet::new();
    let mut current_row_caret_indexes = HashSet::new();
    let current_row = &mut tachyom_manifold[current_row_index];
    for prev_row_caret_index in prev_row_caret_indexes {
        current_row[prev_row_caret_index] = '.';
    }
    for idx in 0..current_row.len() {
        if current_row[idx] == '.' {
            if prev_row_pipe_indexes.contains(&idx) {
                current_row_pipe_indexes.insert(idx);
                current_row[idx] = '|';
            }
        } else if current_row[idx] == '^' {
            current_row_caret_indexes.insert(idx);
            let (left, right) = current_row.split_at_mut(idx);
            if prev_row_pipe_indexes.contains(&idx) {
                *ans += 1;
            }
            if idx > 0 {
                if left[idx - 1] == '.' {
                    current_row_pipe_indexes.insert(idx - 1);
                    left[idx - 1] = '|';
                }
            }
            if right.len() > 1 {
                if right[1] == '.' {
                    current_row_pipe_indexes.insert(idx + 1);
                    right[1] = '|';
                }
            }
        }
    }

    return first_postprocessing(
        tachyom_manifold,
        current_row_index + 1,
        current_row_pipe_indexes,
        current_row_caret_indexes,
        ans,
    );
}
