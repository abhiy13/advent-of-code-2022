use anyhow::Result;

use crate::reader::reader;

const MAX_SUMS: usize = 3;

pub fn solve() -> Result<()> {

    let lines = reader::read();
    let mut sums: Vec<i32> = Vec::new();
    let mut sum = 0;
    for line in lines {
        if line.len() == 0 {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>()?;
        }
    }

    sums.sort();
    sums.reverse();

    let mut max_sum = 0;
    for i in 0..MAX_SUMS {
        max_sum += sums[i];
    }

    // println!("{:?}", sums);
    println!("{:?}", max_sum);
    Ok(())

}