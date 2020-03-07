use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;

const EPSILON: f32 = 0.0001f32;

pub fn pick_key<'a, TK, TV>(rng: &mut ThreadRng, map: &'a HashMap<TK, TV>) -> &'a TK {
    let mut keys = map.keys();
    let idx = rng.gen_range(0, keys.len());
    keys.nth(idx).unwrap()
}

pub fn temperature_sample<'a, TK: Eq + Hash>(
    rng: &mut ThreadRng,
    map: &'a HashMap<TK, usize>,
    temperature: f32,
) -> Option<&'a TK> {
    /*
        Hopefully the right reimplementation of:

        preds = np.log(preds + EPSILON) / temperature
        exp_preds = np.exp(preds)
        preds = exp_preds / np.sum(exp_preds)
        probas = np.random.multinomial(1, preds, 1)
        index = np.argmax(probas)
    */
    let prob_tot = map.values().sum::<usize>() as f32;
    let exp_preds: HashMap<&TK, f32> = map
        .iter()
        .map(|(key, n)| {
            let n_norm = *n as f32 / prob_tot;
            let sam: f32 = ((n_norm + EPSILON) / temperature).log(2f32).exp();
            (key, sam)
        })
        .collect();
    let exp_sum: f32 = exp_preds.values().sum();
    let preds: HashMap<&TK, f32> = exp_preds
        .iter()
        .map(|(key, n)| (*key, n / exp_sum))
        .collect();
    let target: f32 = rng.gen();
    let mut acc: f32 = 0f32;
    for (s, prob) in preds {
        acc += prob;
        if acc >= target {
            return Some(s);
        }
    }
    None
}
