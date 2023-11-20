use bit_set::BitSet;
use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

struct BfsState {
    pos: Pos,
    dist: u32,
}

fn find_optimal_route_len(grid: &Vec<Vec<u8>>, start: Pos, end: Pos) -> Option<u32> {
    let mut q: VecDeque<BfsState> = VecDeque::new();
    q.push_back(BfsState {
        pos: start,
        dist: 0,
    });

    let mut visited: Vec<BitSet> = grid.iter().map(|_| BitSet::new()).collect();

    loop {
        if let Some(BfsState { pos, dist }) = q.pop_front() {
            if pos == end {
                return Some(dist);
            }
            if visited[pos.y].contains(pos.x) {
                continue;
            }
            visited[pos.y].insert(pos.x);

            let here = grid[pos.y][pos.x];

            if pos.x > 0 && grid[pos.y][pos.x - 1] <= here + 1 {
                q.push_back(BfsState {
                    pos: Pos {
                        x: pos.x - 1,
                        y: pos.y,
                    },
                    dist: dist + 1,
                });
            }
            if pos.x < grid[0].len() - 1 && grid[pos.y][pos.x + 1] <= here + 1 {
                q.push_back(BfsState {
                    pos: Pos {
                        x: pos.x + 1,
                        y: pos.y,
                    },
                    dist: dist + 1,
                });
            }
            if pos.y > 0 && grid[pos.y - 1][pos.x] <= here + 1 {
                q.push_back(BfsState {
                    pos: Pos {
                        x: pos.x,
                        y: pos.y - 1,
                    },
                    dist: dist + 1,
                });
            }
            if pos.y < grid.len() - 1 && grid[pos.y + 1][pos.x] <= here + 1 {
                q.push_back(BfsState {
                    pos: Pos {
                        x: pos.x,
                        y: pos.y + 1,
                    },
                    dist: dist + 1,
                });
            }
        } else {
            return None;
        }
    }
}

pub fn main() {
    println!("Hello, world!");

    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut end: Option<Pos> = None;

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line.is_empty() {
            break;
        }

        if let Some(idx) = line.find('E') {
            end = Some(Pos {
                x: idx,
                y: grid.len(),
            });
        }

        grid.push(
            line.chars()
                .map(|c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    c => c,
                })
                .map(|c| c as u8 - b'a')
                .collect(),
        );
    }

    let mut shortest_path: Option<u32> = None;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 0 {
                continue;
            }

            if let Some(result) = find_optimal_route_len(&grid, Pos { x, y }, end.unwrap()) {
                if let Some(best) = shortest_path {
                    if result < best {
                        shortest_path = Some(result);
                    }
                } else {
                    shortest_path = Some(result);
                }
            }
        }
    }

    println!("{}", shortest_path.unwrap());
}
