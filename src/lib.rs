mod syllable;

use clap::ValueEnum;
use rand::Rng;
use rand_distr::{Distribution, Poisson};

use crate::syllable::Syllable;

pub struct NameGenerator {
    min_length: u32,
    max_length: Option<u32>,
    syllable_count: SyllableCountDistribution,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Script {
    Arabic,
    Ascii,
    Cyrillic,
    Devanagari,
    Greek,
    Gujarati,
    Hangul,
    Kannada,
    Hebrew,
    Latin,
    LatinTitleCase,
    Syllabics,
    Shavian,
    Hiragana,
    Katakana,
    Futhark,
    Gothic,
    Ogham,
    Georgian,
    Orkhon,
}

struct Name {
    syllables: Vec<Syllable>,
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

    pub fn generate(&self, rng: &mut impl Rng, script: Script) -> String {
        loop {
            let name = Name::random(rng, &self.syllable_count);
            if name.len() >= self.min_length as usize
                && self.max_length.is_none_or(|max| name.len() <= max as usize)
            {
                return name.write(script);
            }
        }
    }
}

impl Name {
    fn random(rng: &mut impl Rng, syllable_distribution: &SyllableCountDistribution) -> Self {
        let syllable_count = match *syllable_distribution {
            SyllableCountDistribution::Fixed(syllables) => syllables,
            SyllableCountDistribution::Poisson(min_syllables, poisson) => {
                min_syllables + poisson.sample(rng) as u32
            }
        };

        let mut syllables = Vec::new();
        for _ in 0..syllable_count {
            let next = Syllable::new(rng, syllables.last_mut());
            syllables.push(next);
        }

        Name { syllables }
    }

    fn len(&self) -> usize {
        self.syllables.iter().map(|syllable| syllable.len()).sum()
    }

    fn write(&self, script: Script) -> String {
        let mut buf = String::new();
        for (i, syllable) in self.syllables.iter().enumerate() {
            let prev = if i == 0 {
                None
            } else {
                Some(&self.syllables[i - 1])
            };

            let next = if i == self.syllables.len() - 1 {
                None
            } else {
                Some(&self.syllables[i + 1])
            };

            syllable.write(&mut buf, script, prev, next);
        }

        buf
    }
}
