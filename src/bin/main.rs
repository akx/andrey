#[macro_use]
extern crate clap;

use andrey::read::generate_map;
use andrey::sampling::{pick_key, temperature_sample};
use andrey::types::{CounterMap, Options};

fn sample_forever(options: &Options, map: &CounterMap, temperature: f32) {
    let mut rng = rand::thread_rng();
    let mut head = pick_key(&mut rng, &map).to_owned();
    loop {
        // if we reach a state where the current head is not in the map, start a new line
        while !map.contains_key(&head) {
            //            eprintln!("Not finding {:?}", head);
            println!();
            head = "".to_owned();
            //head = pick_key(&mut rng, &map).to_owned();
        }
        let tails = map.get(&head).expect("tail found");
        let ch = temperature_sample(&mut rng, tails, temperature).expect("sample ok");
        //eprintln!("[{:?}] => ch {:?}", head, ch);
        if *ch != '\0' {
            print!("{}", ch);
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
    )
    .get_matches();
    let opt: Options = Options {
        state_size: value_t!(matches, "state_size", usize).expect("valid integer"),
    };
    let map = generate_map(&opt, matches.value_of("input").expect("got input")).expect("worked");
    let temperature = value_t!(matches, "temperature", f32).expect("valid float");

    sample_forever(&opt, &map, temperature);
}
