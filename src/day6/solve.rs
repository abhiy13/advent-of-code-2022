use anyhow::Result;

use crate::reader::reader;

// set this as 4 for part 1;
const NUM_CONTINUOUS_DISTINCT: u32 = 14;

pub fn solve() -> Result<()> {

    let lines = reader::read();

    let input = lines.get(0).unwrap();

    let check_unique = |x: u32| {
        assert!(x >= 3);
        let mut mask: i32 = 0;
        let start = x + 1 - NUM_CONTINUOUS_DISTINCT;
        for i in start..x+1 {
            mask |= 1 << input.chars().nth(i as usize).unwrap() as i32 - 'a' as i32;
        }
        // println!("{:b}", mask);
        return i32::count_ones(mask) == NUM_CONTINUOUS_DISTINCT;
    };

    for i in NUM_CONTINUOUS_DISTINCT - 1..input.len() as u32 {
        if check_unique(i) {
            println!("{}", i + 1);
            break;
        }
    }

    Ok(())
}