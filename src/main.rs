use rand::prelude::*;

fn main() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    for _ in 0..100 {
        let name = name(&mut rng);
        if name.len() != 1 {
            println!("{}", name);
        }
    }
}

fn name(rng: &mut impl Rng) -> String {
    let len = [1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 6]
        .choose(rng)
        .unwrap();

    let mut name = String::new();
    initial(&mut name, rng);
    for _ in 0..(len - 1) {
        syllable(&mut name, rng);
    }

    name
}

fn initial(buf: &mut String, rng: &mut impl Rng) {
    if rng.gen_bool(0.25) {
        nucleus(buf, rng)
    } else {
        syllable(buf, rng)
    }
}

fn syllable(buf: &mut String, rng: &mut impl Rng) {
    let last = buf.chars().last();
    let onset = ['p', 't', 'k', 's', 'm', 'n', 'l', 'j', 'w']
        .choose_weighted(rng, |v| match v {
            'p' => 61,
            't' => 45,
            'k' => 91,
            's' => 64,
            'm' if last != Some('n') => 50,
            'n' if last != Some('n') => 32,
            'l' => 83,
            'j' => 35,
            'w' => 34,
            _ => 0,
        })
        .unwrap();
    buf.push(*onset);

    nucleus(buf, rng);
}

fn nucleus(buf: &mut String, rng: &mut impl Rng) {
    let last = buf.chars().last();
    let nucleus = ['a', 'i', 'e', 'o', 'u']
        .choose_weighted(rng, |v| match v {
            'a' => 146,
            'e' => 94,
            'i' if last != Some('t') && last != Some('j') => 109,
            'o' if last != Some('w') => 82,
            'u' if last != Some('w') => 60,
            _ => 0,
        })
        .unwrap();

    buf.push(*nucleus);

    if rng.gen_bool(0.06) {
        buf.push('n');
    }
}
