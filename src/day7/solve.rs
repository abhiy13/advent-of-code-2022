use std::collections::HashMap;
use anyhow::Result;

use crate::reader::reader;


pub fn solve() -> Result<()> {

    let lines = reader::read();

    let mut directory_size: HashMap<String, i64> = HashMap::new();
    let mut directory_stack: Vec<String> = Vec::new();

    for line in lines {
        let console: Vec<&str> = line.split(" ").collect();
        if *console.get(0).unwrap() == "$" { // Command
            match *console.get(1).unwrap() {
                "cd" => {
                    match *console.get(2).unwrap() {
                        ".." => {
                            directory_stack.pop().unwrap();
                        }
                        dir => {
                            directory_stack.push(dir.into());
                        }
                    }
                }
                _ => {}
            }
        } else if *console.get(0).unwrap() != "dir" {
            let file_size = console.get(0).unwrap().parse::<i64>()?;
            let mut curr_dir = String::new();

            // Update all parents dir size
            for (_, x) in directory_stack.iter().enumerate() {
                curr_dir.push_str(x);
                curr_dir.push_str("/");
                let mut val = 0;
                {
                    val = *directory_size.get(curr_dir.clone().as_str()).unwrap_or(&0);
                }
                directory_size.insert(curr_dir.clone(), val + file_size);
            }
        }
        // println!("{:?} {:?}", console, directory_stack);
    }


    // println!("{:?}", directory_size);

    // Part 1
    println!("{}", directory_size.values().filter(|x| **x <= 100_000).sum::<i64>());

    const TOTAL_DISK_SIZE: i64 = 70_000_000;
    const REQUIRED_SIZE: i64 = 30_000_000;

    let free_space = TOTAL_DISK_SIZE - *directory_size.get("//").unwrap();

    // Part 2
    println!("{}, {}", *directory_size.get("//").unwrap(), free_space);
    println!("{:?}", directory_size.values().filter(|x| (free_space + **x) >= REQUIRED_SIZE).min().unwrap());

    Ok(())
}