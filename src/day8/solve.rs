use std::collections::{HashMap, HashSet};
use anyhow::Result;

use crate::reader::reader;

pub fn solve() -> Result<()> {

    let lines = reader::read();

    let mut sum = 0;
    let m = lines.get(0).unwrap().len();
    let n = lines.len();

    println!("{}, {}", n, m);
    let check = |x: usize, y: usize| -> i32 {
        let val = lines.get(x).unwrap()
            .chars().nth(y).unwrap()
            .to_digit(10).unwrap() as i32;

        let mut dist = 1;
        let mut c_dist = 0;
        for i in (0..x).rev() {
            c_dist += 1;
            if lines.get(i).unwrap()
                .chars().nth(y).unwrap()
                .to_digit(10).unwrap() as i32 >= val {
                break;
            }
        }

        dist *= c_dist;
        c_dist = 0;

        for i in x+1..n {
            c_dist += 1;
            if lines.get(i).unwrap()
                .chars().nth(y).unwrap()
                .to_digit(10).unwrap() as i32 >= val {
                break;
            }
        }

        dist *= c_dist;
        c_dist = 0;

        for i in (0..y).rev() {
            c_dist += 1;
            if lines.get(x).unwrap()
                .chars().nth(i).unwrap()
                .to_digit(10).unwrap() as i32 >= val {
                break;
            }
        }

        dist *= c_dist;
        c_dist = 0;

        for i in y+1..m {
            c_dist += 1;
            if lines.get(x).unwrap()
                .chars().nth(i).unwrap()
                .to_digit(10).unwrap() as i32 >= val {
                break;
            }
        }
        dist *= c_dist;

        dist
    };

    for i in 0..n {
       for j in 0..m {
           let x = check(i, j);
           if sum < x {
               sum = x;
           }
       }
        // println!()
    }

    // println!("{:?}", vis.len());
    // println!("{:?}", vis);
    println!("{:?}", sum);
    Ok(())
}
