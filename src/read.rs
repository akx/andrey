use crate::types::{CounterMap, Options};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn add(map: &mut CounterMap, head: String, next: char) {
    //    eprintln!("[{:?}] => {:?}", head, next);
    map.entry(head)
        .or_insert_with(HashMap::new)
        .entry(next)
        .and_modify(|e| *e += 1)
        .or_insert(1);
}

fn add_from_trail(map: &mut CounterMap, trail: &VecDeque<char>, next: char) {
    let head: String = trail.clone().into_iter().collect();
    add(map, head.trim_start_matches('\0').to_owned(), next);
}

pub fn generate_map(options: &Options, filename: &str) -> io::Result<CounterMap> {
    let mut map: CounterMap = CounterMap::new();
    let f = File::open(filename)?;
    let f = BufReader::new(f);
    for line in f.lines() {
        let mut trail: VecDeque<char> = VecDeque::with_capacity(options.state_size);
        for ch in line?.chars() {
            add_from_trail(&mut map, &trail, ch);
            if trail.len() >= options.state_size {
                trail.pop_front();
            }
            trail.push_back(ch);
        }
        //        add_from_trail(&mut map, &trail, '\0');
    }
    Ok(map)
}
