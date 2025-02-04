use rand::{seq::IndexedRandom, Rng};
use rand_distr::{Distribution, Poisson};

pub struct NameGenerator {
    min_length: u32,
    max_length: Option<u32>,
    syllable_count: SyllableCountDistribution,
}

enum SyllableCountDistribution {
    Fixed(u32),
    Poisson(u32, Poisson<f64>),
}

impl NameGenerator {
    pub fn new(min_length: u32, max_length: Option<u32>) -> Self {
        let min_syllables = min_length.div_ceil(3);
        let max_syllables = match max_length {
            Some(max) => (max + 1) / 2,
            None => (min_length + 7) / 2,
        };

        let syllable_count = if min_syllables == max_syllables {
            SyllableCountDistribution::Fixed(min_syllables)
        } else {
            let variance = (max_syllables - min_syllables) as f64;
            SyllableCountDistribution::Poisson(
                min_syllables,
                Poisson::new(variance.min(Poisson::<f64>::MAX_LAMBDA)).unwrap(),
            )
        };

        NameGenerator {
            min_length,
            max_length,
            syllable_count,
        }
    }

    pub fn generate(&self, rng: &mut impl Rng, title_case: bool) -> String {
        loop {
            let name = name(rng, &self.syllable_count, title_case);
            if name.len() >= self.min_length as usize
                && self.max_length.is_none_or(|max| name.len() <= max as usize)
            {
                return name;
            }
        }
    }
}

fn name(rng: &mut impl Rng, syllable_distribution: &SyllableCountDistribution, title_case: bool) -> String {
    let syllables = match *syllable_distribution {
        SyllableCountDistribution::Fixed(syllables) => syllables,
        SyllableCountDistribution::Poisson(min_syllables, poisson) => {
            min_syllables + poisson.sample(rng) as u32
        }
    };

    let mut name = String::new();
    initial(&mut name, rng, title_case);
    for _ in 0..(syllables - 1) {
        syllable(&mut name, rng, false);
    }

    name
}

fn initial(buf: &mut String, rng: &mut impl Rng, title_case: bool) {
    if rng.random_bool(0.25) {
        nucleus(buf, rng, title_case)
    } else {
        syllable(buf, rng, title_case)
    }
}

fn syllable(buf: &mut String, rng: &mut impl Rng, title_case: bool) {
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

    if title_case {
        buf.push(onset.to_ascii_uppercase());
    } else {
        buf.push(*onset);
    }

    nucleus(buf, rng, false);
}

fn nucleus(buf: &mut String, rng: &mut impl Rng, title_case: bool) {
    let last = buf.chars().last();
    let nucleus = ['a', 'i', 'e', 'o', 'u']
        .choose_weighted(rng, |v| match v {
            'a' => 146,
            'e' => 94,
            'i' if last != Some('t') && last != Some('j') => 140,
            'o' if last != Some('w') => 88,
            'u' if last != Some('w') => 64,
            _ => 0,
        })
        .unwrap();

    if title_case {
        buf.push(nucleus.to_ascii_uppercase());
    } else {
        buf.push(*nucleus);
    }

    if rng.random_bool(0.06) {
        buf.push('n');
    }
}
