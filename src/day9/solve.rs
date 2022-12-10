use std::collections::{HashSet};

use anyhow::Result;

use crate::reader::reader;

// change to 2 for Part 1
const TAIL_LENGTH: usize = 10;

pub fn solve() -> Result<()> {
    let lines = reader::read();
    let mut locations: HashSet<(i32, i32)> = HashSet::new();
    let mut px = [1000i32; TAIL_LENGTH];
    let mut py = [0i32; TAIL_LENGTH];

    let _ = lines.iter()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|cmds| (*cmds.get(0).unwrap(), cmds.get(1).unwrap().parse::<i32>().unwrap()))
        .map(|(cmd, moves)| {
            let (dx, dy) = match cmd {
                "L" => (0, -1),
                "R" => (0, 1),
                "U" => (-1, 0),
                "D" => (1, 0),
                _ => unreachable!()
            };

            for _ in 0..moves {
                px[0] += dx; py[0] += dy;
                for it in 1..TAIL_LENGTH {
                    let (x, y, mut ax, mut ay) = (px[it - 1], py[it - 1], px[it], py[it]);
                    let mut minc = i32::abs(x - ax) + i32::abs(y - ay);
                    let (cax, cay) = (ax, ay);
                    let mut reach = false;
                    for i in -1..2 {
                        for j in -1..2 {
                            if cax + i == x && cay + j == y {
                                reach = true;
                            }
                            let mabs = i32::abs(x - cax - i) + i32::abs(y - cay - j);
                            if mabs < minc {
                                ax = cax + i;
                                ay = cay + j;
                                minc = mabs;
                            }
                        }
                    }
                    if reach {
                        ax = cax;
                        ay = cay
                    }
                    px[it] = ax;
                    py[it] = ay;
                    // println!("{} {}, {} {}", x, y, px[0], py[0]);
                }
                locations.insert((px[TAIL_LENGTH - 1], py[TAIL_LENGTH - 1]));
            }
        }).collect::<()>();

    println!("{:?}", locations);
    println!("{}", locations.len());

    Ok(())
}