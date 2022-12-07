use anyhow::Result;

use crate::reader::reader;

pub fn solve() -> Result<()> {

    let lines = reader::read();

    let mut count = 0;
    for line in lines {
        let mut pairs: Vec<i32> = line.split(",")
            .into_iter()
            .map(|x| x.split("-"))
            .flatten()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        for _ in 0..2 {
            // Part 1
            // Complete Overlap
            if pairs[0] <= pairs[2] &&
                pairs[1] >= pairs[3] {
                count += 1;
                break;
            }

            // Part 2, Or Partial Overlap
            if pairs[0] <= pairs[2] &&
                pairs[2] <= pairs[1] {
                count += 1;
                break;
            }
            // Swap values
            pairs.swap(0, 2);
            pairs.swap(1, 3);
        }
    }

    println!("{}", count);
    Ok(())
}