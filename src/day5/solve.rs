use std::collections::VecDeque;
use anyhow::Result;

use crate::reader::reader;

const MAX_CRATES: i32 = 9;

pub fn solve() -> Result<()> {

    let lines = reader::read();

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    for __ in 0..MAX_CRATES {
        stacks.push(VecDeque::new());
    }

    let mut iter = lines.iter();
    while let Some(line) = iter.next() {
        let elems: Vec<char> = line.chars().into_iter().collect();
        if let Some(_) = elems.get(1).unwrap().to_digit(10) {
            break;
        }
        for i in (1..elems.len()).step_by(4) {
            if *elems.get(i).unwrap() == ' ' {
                continue;
            }
            stacks[i / 4].push_back(*elems.get(i).unwrap());
        }
    }

    eprintln!("{:?}", stacks);
    iter.next(); // empty line

    while let Some(line) = iter.next() {
        let elems: Vec<&str> = line.split(" ").collect();
        println!("{:?}", elems);
        let mut count = elems[1].parse::<i32>()?;
        let from = elems[3].parse::<usize>()? - 1; // zero indexed
        let to = elems[5].parse::<usize>()? - 1; // zero indexed
        assert!(stacks.get(from).unwrap().len() >= count as usize);
        // Part 1
        /*
        while count > 0 {
            let x = stacks.get_mut(from).unwrap().pop_front().unwrap();
            stacks.get_mut(to).unwrap().push_front(x);
            count -= 1;
        }
         */
        // Part 2
        let mut items_to_move: Vec<char> = Vec::new();
        for _ in 0..count {
            items_to_move.push(stacks.get_mut(from).unwrap().pop_front().unwrap());
        }
        items_to_move.reverse();
        for elem in items_to_move {
            stacks.get_mut(to).unwrap().push_front(elem);
        }
    }

    eprintln!("{:?}", stacks);

    for i in 0..MAX_CRATES {
        if stacks.get(i as usize).unwrap().len() == 0 {
            break;
        }
        print!("{}", stacks.get(i as usize).unwrap().front().unwrap());
    }

    Ok(())
}