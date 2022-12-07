use anyhow::Result;

use crate::reader::reader;

pub fn solve() -> Result<()> {

    let lines = reader::read();

    let mut score = 0;
    for line in lines {
        let xdd: Vec<&str> = line.split(" ").into_iter().collect();
        assert_eq!(xdd.len(), 2);
        // Part 1 solution
        // score += calculate_score(xdd[0], xdd[1]);
        // Part 2
        score += calculate_score_ideal(xdd[0], xdd[1]);
    }

    println!("{}", score);
    Ok(())
}

fn calculate_score_ideal(a: &str, b: &str) -> i32 {
    match a {
        "A" => {
            match b {
                "X" => 0 + 3,
                "Y" => 3 + 1,
                "Z" => 6 + 2,
                _ => panic!("xdd")
            }
        },
        "B" => {
            match b {
                "X" => 0 + 1,
                "Y" => 3 + 2,
                "Z" => 6 + 3,
                _ => panic!("xdd")
            }
        },
        "C" => {
            match b {
                "X" => 0 + 2,
                "Y" => 3 + 3,
                "Z" => 6 + 1,
                _ => panic!("xdd")
            }
        }
        _ => panic!("xdd")
    }
}

fn calculate_score(a: &str, b: &str) -> i32 {
    match a {
        "A" => {
            match b {
                "X" => 3 + 1,
                "Y" => 6 + 2,
                "Z" => 0 + 3,
                _ => panic!("xdd")
            }
        },
        "B" => {
            match b {
                "X" => 0 + 1,
                "Y" => 3 + 2,
                "Z" => 6 + 3,
                _ => panic!("xdd")
            }
        },
        "C" => {
            match b {
                "X" => 6 + 1,
                "Y" => 0 + 2,
                "Z" => 3 + 3,
                _ => panic!("xdd")
            }
        }
        _ => panic!("xdd")
    }
}

