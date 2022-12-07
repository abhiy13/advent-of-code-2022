use std::collections::{HashMap, HashSet};
use anyhow::Result;

use crate::reader::reader;

pub fn solve() -> Result<()> {
    let mut char_values: HashMap<char, u32> = HashMap::new();
    for i in 0..26 {
        char_values.insert(char::from_u32('a' as u32 + i).unwrap(), i + 1);
        char_values.insert(char::from_u32('A' as u32 + i).unwrap(), 26 + i + 1);
    }

    let mut sum = 0;

    let lines = reader::read();

    // Part 1
    /*
    for line in lines {
        let mut vMap: HashMap<char, bool> = HashMap::new();
        let len = line.len();
        for i in 0..len/2 {
            vMap.insert(line.chars().nth(i).unwrap(), true);
        }
        for i in 0..len/2 {
            let ch = line.chars().nth(line.len() - i - 1).unwrap();
            if let Some(_) = vMap.get(&ch) {
                sum += char_values.get(&ch).unwrap();
                // Remove this so that we dont count multiple times
                vMap.remove(&ch);
            }
        }
    }
    */

    let mut line_num = 0;
    while line_num < lines.len() {
        let mut count_map: HashMap<char, i32> = HashMap::new();
        let line = &lines[line_num..line_num+3];
        for iter in 0..3 {
            let ln: HashSet<char> = line[iter].chars().into_iter().collect();
            for curr_char in ln.iter() {
                count_map.insert(*curr_char, *count_map.get(curr_char).unwrap_or(&0) + 1);
            }
        }
        for (key, val) in count_map.iter() {
            if *val == 3 {
                sum += char_values.get(key).unwrap();
            }
        }
        line_num += 3;
    }

    println!("{}", sum);
    Ok(())
}