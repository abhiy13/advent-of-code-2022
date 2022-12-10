use anyhow::Result;

use crate::reader::reader;

const MAX_CYCLES: usize = 241;
const VALS: [i32; 6] = [20, 60, 100, 140, 180, 220];

pub fn solve() -> Result<()> {
    let lines = reader::read();

    let mut diff = [0i32; MAX_CYCLES];
    diff[0] = 1;
    let mut curr_cycle = 1;

    let _ = lines.iter()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .map(|x| {
            match *x.get(0).unwrap() {
                "noop" => curr_cycle += 1,
                "addx" => {
                    curr_cycle += 2;
                    if curr_cycle < MAX_CYCLES {
                        diff[curr_cycle] += x.get(1).unwrap().parse::<i32>().unwrap();
                    }
                }
                _ => unreachable!()
            }
        })
        .collect::<()>();

    for i in 1..MAX_CYCLES {
        diff[i] += diff[i - 1];
    }

    let mut sum = 0;
    for x in VALS {
        sum += diff[x as usize] * x;
    }

    // Part 2 solution
    for i in 0..MAX_CYCLES - 1 {
        if i % 40 == 0 {
            println!();
        }
        if i32::abs(diff[i + 1] - (i % 40) as i32) <= 1 {
            print!("#")
        } else {
            print!(".")
        }
    }

    // Part 1 solution
    println!("\n{}", sum);
    Ok(())
}