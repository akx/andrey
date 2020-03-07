use std::collections::HashMap;

pub type TailMap = HashMap<char, usize>;
pub type CounterMap = HashMap<String, TailMap>;

pub struct Options {
    pub state_size: usize,
}
