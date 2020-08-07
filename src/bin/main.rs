#[macro_use]
extern crate clap;

use andrey::read::generate_map;
use andrey::sampling::{pick_key, temperature_sample};
use andrey::types::{CounterMap, Options};

fn sample_forever(
    options: &Options,
    map: &CounterMap,
    temperature: f32,
    max_len: u32,
    pick_random_key: bool,
) {
    let mut rng = rand::thread_rng();
    let mut head = pick_key(&mut rng, &map).to_owned();
    let mut count: u32 = 0;
    loop {
        // if we reach a state where the current head is not in the map, start a new line
        if (max_len > 0 && count > max_len) || !map.contains_key(&head) {
            while !map.contains_key(&head) {
                head = if pick_random_key {
                    pick_key(&mut rng, &map).to_owned()
                } else {
                    "".to_owned()
                };
            }
            println!();
            count = 0;
        }
        let tails = map.get(&head).expect("tail found");
        let ch = temperature_sample(&mut rng, tails, temperature).expect("sample ok");
        //eprintln!("[{:?}] => ch {:?}", head, ch);
        if *ch != '\0' {
            print!("{}", ch);
            count += 1;
        }
        head.push(*ch);
        if head.len() > options.state_size {
            head = head.chars().skip(1).collect();
        }
    }
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: crate_version!())
        (author: crate_authors!())
        (@arg state_size: -s +takes_value +required "State size")
        (@arg temperature: -t +takes_value +required "Sampling temperature")
        (@arg input: -i +takes_value +required "Input file")
        (@arg max_len: -l +takes_value "Maximum line length")
        (@arg pick_random_key: -r "Pick random key when switching lines")
    )
    .get_matches();
    let opt: Options = Options {
        state_size: value_t!(matches, "state_size", usize).expect("valid integer"),
    };
    let map = generate_map(&opt, matches.value_of("input").expect("got input")).expect("worked");
    let temperature = value_t!(matches, "temperature", f32).expect("valid float");
    let max_len = value_t!(matches, "max_len", u32).unwrap_or(0);
    let pick_random_key = matches.is_present("pick_random_key");
    sample_forever(&opt, &map, temperature, max_len, pick_random_key);
}
