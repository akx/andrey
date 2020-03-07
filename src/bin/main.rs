use andrey::read::generate_map;
use andrey::sampling::{pick_key, temperature_sample};
use andrey::types::Options;

fn main() {
    let opt: Options = Options { state_size: 3 };
    let map = generate_map(&opt, "test.txt").expect("worked");
    let mut rng = rand::thread_rng();
    let mut head = pick_key(&mut rng, &map).to_owned();
    let temperature = 6.3f32;
    loop {
        // if we reach a state where the current head is not in the map, start a new line
        while !map.contains_key(&head) {
            println!();
            head = pick_key(&mut rng, &map).to_owned();
        }
        let tails = map.get(&head).expect("tail found");
        let ch = temperature_sample(&mut rng, tails, temperature).expect("sample ok");
        if *ch != '\0' {
            print!("{}", ch);
        }
        head.push(*ch);
        head = head.chars().skip(1).collect();
    }
}
