use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "./input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect())
        .collect();
    first(&grid);
    let mut second_ans = 0;
    second(&mut grid, &mut second_ans);
    println!("answer is this {}", second_ans);
    Ok(())
}

fn first(grid: &[Vec<char>]) {
    let mut ans = 0;
    let rows = grid.len();
    for i in 0..rows {
        let cols = grid[i].len();
        for j in 0..cols {
            if grid[i][j] == '.' {
                continue;
            }
            let mut count = 0;
            static NEIGHBORS: &[(isize, isize)] = &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            for &(di, dj) in NEIGHBORS {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni >= 0
                    && (ni as usize) < rows
                    && nj >= 0
                    && (nj as usize) < grid[ni as usize].len()
                {
                    if grid[ni as usize][nj as usize] == '@' {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn second<'a>(grid: &'a mut Vec<Vec<char>>, second_ans: &'a mut usize) -> &'a mut usize {
    let rows = grid.len();
    let mut ans = 0;
    for i in 0..rows {
        let cols = grid[i].len();
        for j in 0..cols {
            if grid[i][j] == '.' {
                continue;
            }
            let mut count = 0;
            static NEIGHBORS: &[(isize, isize)] = &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            for &(di, dj) in NEIGHBORS {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni >= 0
                    && (ni as usize) < rows
                    && nj >= 0
                    && (nj as usize) < grid[ni as usize].len()
                {
                    if grid[ni as usize][nj as usize] == '@' {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                *second_ans += 1;
                ans += 1;
                grid[i][j] = '.';
            }
        }
    }
    if ans != 0 {
        second(grid, second_ans);
    }
    return second_ans;
}
