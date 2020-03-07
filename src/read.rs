use crate::types::{CounterMap, Options};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn generate_map(options: &Options, filename: &str) -> io::Result<CounterMap> {
    let mut map: CounterMap = CounterMap::new();
    let f = File::open(filename)?;
    let f = BufReader::new(f);
    for line in f.lines() {
        let mut trail: VecDeque<char> = VecDeque::with_capacity(options.state_size);
        for _ in 0..options.state_size {
            trail.push_back('\0');
        }
        for ch in line?.chars() {
            let head: String = trail.clone().into_iter().collect();
            trail.pop_front();
            trail.push_back(ch);
            map.entry(head)
                .or_insert_with(HashMap::new)
                .entry(ch)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
    Ok(map)
}
