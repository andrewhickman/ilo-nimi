use rand::{seq::IndexedRandom, Rng};

use crate::Script;

#[derive(Debug, Copy, Clone)]
pub struct Syllable {
    onset: Onset,
    nucleus: Nucleus,
    coda: Coda,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Onset {
    Null,
    P,
    T,
    K,
    S,
    M,
    N,
    L,
    J,
    W,
}

#[derive(Debug, Copy, Clone)]
pub enum Nucleus {
    A,
    E,
    I,
    O,
    U,
}

#[derive(Debug, Copy, Clone)]
pub enum Coda {
    Null,
    N,
}

impl Syllable {
    pub fn new(rng: &mut impl Rng, prev: Option<&mut Syllable>) -> Self {
        let onset = if let Some(prev) = prev {
            let onset = Onset::random(rng);
            if matches!(onset, Onset::M | Onset::N) {
                prev.coda = Coda::Null;
            }
            onset
        } else if rng.random_bool(0.25) {
            Onset::Null
        } else {
            Onset::random(rng)
        };

        let nucleus = Nucleus::random(rng, onset);

        let coda = Coda::random(rng);

        Syllable {
            onset,
            nucleus,
            coda,
        }
    }

    pub fn len(&self) -> usize {
        self.onset.len() + self.nucleus.len() + self.coda.len()
    }

    pub fn write(&self, buf: &mut String, script: Script, first: bool, last: bool) {
        match script {
            Script::LatinTitleCase if first => {
                if let Some(onset) = self.onset.latin_char() {
                    buf.push(onset.to_ascii_uppercase());
                    buf.push(self.nucleus.latin_char());
                } else {
                    buf.push(self.nucleus.latin_char().to_ascii_uppercase());
                }
                buf.extend(self.coda.latin_char());
            }
            Script::Latin | Script::LatinTitleCase => {
                buf.extend(self.onset.latin_char());
                buf.push(self.nucleus.latin_char());
                buf.extend(self.coda.latin_char());
            }
            Script::Cyrillic => {
                if self.onset == Onset::J {
                    buf.push(self.nucleus.iotated_cyrillic_char());
                } else {
                    buf.extend(self.onset.cyrillic_char());
                    buf.push(self.nucleus.cyrillic_char());
                }

                buf.extend(self.coda.cyrillic_char());
            }
            Script::Greek => {
                buf.extend(self.onset.greek_char());
                buf.push(self.nucleus.greek_char());
                buf.extend(self.coda.greek_char());
            }
            Script::Hebrew => {
                buf.push(self.onset.hebrew_char());
                buf.push(self.nucleus.hebrew_char());
                buf.extend(self.coda.hebrew_char(last));
            }
            Script::Devanagari => {
                if let Some(onset) = self.onset.devanagari_char() {
                    buf.push(onset);
                    buf.extend(self.nucleus.devanagari_modifier_char());
                } else {
                    buf.push(self.nucleus.devanagari_char());
                }
                buf.extend(self.coda.devanagari_char());
            }
            Script::Hangul => {
                buf.push(self.hangul_char());
            }
            Script::Arabic => {
                buf.push(self.onset.arabic_char());
                buf.extend(self.nucleus.arabic_chars(self.coda));
            }
        }
    }

    fn hangul_char(&self) -> char {
        match (self.onset, self.nucleus, self.coda) {
            (Onset::Null, Nucleus::A, Coda::Null) => '아',
            (Onset::Null, Nucleus::E, Coda::Null) => '어',
            (Onset::Null, Nucleus::O, Coda::Null) => '오',
            (Onset::Null, Nucleus::U, Coda::Null) => '우',
            (Onset::Null, Nucleus::I, Coda::Null) => '이',
            (Onset::Null, Nucleus::A, Coda::N) => '안',
            (Onset::Null, Nucleus::E, Coda::N) => '언',
            (Onset::Null, Nucleus::O, Coda::N) => '온',
            (Onset::Null, Nucleus::U, Coda::N) => '운',
            (Onset::Null, Nucleus::I, Coda::N) => '인',
            (Onset::J, Nucleus::A, Coda::Null) => '야',
            (Onset::J, Nucleus::E, Coda::Null) => '여',
            (Onset::J, Nucleus::O, Coda::Null) => '요',
            (Onset::J, Nucleus::U, Coda::Null) => '유',
            (Onset::J, Nucleus::I, Coda::Null) => unreachable!(),
            (Onset::J, Nucleus::A, Coda::N) => '얀',
            (Onset::J, Nucleus::E, Coda::N) => '연',
            (Onset::J, Nucleus::O, Coda::N) => '욘',
            (Onset::J, Nucleus::U, Coda::N) => '윤',
            (Onset::J, Nucleus::I, Coda::N) => unreachable!(),
            (Onset::W, Nucleus::A, Coda::Null) => '와',
            (Onset::W, Nucleus::E, Coda::Null) => '워',
            (Onset::W, Nucleus::O, Coda::Null) => unreachable!(),
            (Onset::W, Nucleus::U, Coda::Null) => unreachable!(),
            (Onset::W, Nucleus::I, Coda::Null) => '위',
            (Onset::W, Nucleus::A, Coda::N) => '완',
            (Onset::W, Nucleus::E, Coda::N) => '원',
            (Onset::W, Nucleus::O, Coda::N) => unreachable!(),
            (Onset::W, Nucleus::U, Coda::N) => unreachable!(),
            (Onset::W, Nucleus::I, Coda::N) => '윈',
            (Onset::K, Nucleus::A, Coda::Null) => '가',
            (Onset::K, Nucleus::E, Coda::Null) => '거',
            (Onset::K, Nucleus::O, Coda::Null) => '고',
            (Onset::K, Nucleus::U, Coda::Null) => '구',
            (Onset::K, Nucleus::I, Coda::Null) => '기',
            (Onset::K, Nucleus::A, Coda::N) => '간',
            (Onset::K, Nucleus::E, Coda::N) => '건',
            (Onset::K, Nucleus::O, Coda::N) => '곤',
            (Onset::K, Nucleus::U, Coda::N) => '군',
            (Onset::K, Nucleus::I, Coda::N) => '긴',
            (Onset::N, Nucleus::A, Coda::Null) => '나',
            (Onset::N, Nucleus::E, Coda::Null) => '너',
            (Onset::N, Nucleus::O, Coda::Null) => '노',
            (Onset::N, Nucleus::U, Coda::Null) => '누',
            (Onset::N, Nucleus::I, Coda::Null) => '니',
            (Onset::N, Nucleus::A, Coda::N) => '난',
            (Onset::N, Nucleus::E, Coda::N) => '넌',
            (Onset::N, Nucleus::O, Coda::N) => '논',
            (Onset::N, Nucleus::U, Coda::N) => '눈',
            (Onset::N, Nucleus::I, Coda::N) => '닌',
            (Onset::T, Nucleus::A, Coda::Null) => '다',
            (Onset::T, Nucleus::E, Coda::Null) => '더',
            (Onset::T, Nucleus::O, Coda::Null) => '도',
            (Onset::T, Nucleus::U, Coda::Null) => '두',
            (Onset::T, Nucleus::I, Coda::Null) => unreachable!(),
            (Onset::T, Nucleus::A, Coda::N) => '단',
            (Onset::T, Nucleus::E, Coda::N) => '던',
            (Onset::T, Nucleus::O, Coda::N) => '돈',
            (Onset::T, Nucleus::U, Coda::N) => '둔',
            (Onset::T, Nucleus::I, Coda::N) => unreachable!(),
            (Onset::L, Nucleus::A, Coda::Null) => '라',
            (Onset::L, Nucleus::E, Coda::Null) => '러',
            (Onset::L, Nucleus::O, Coda::Null) => '로',
            (Onset::L, Nucleus::U, Coda::Null) => '루',
            (Onset::L, Nucleus::I, Coda::Null) => '리',
            (Onset::L, Nucleus::A, Coda::N) => '란',
            (Onset::L, Nucleus::E, Coda::N) => '런',
            (Onset::L, Nucleus::O, Coda::N) => '론',
            (Onset::L, Nucleus::U, Coda::N) => '룬',
            (Onset::L, Nucleus::I, Coda::N) => '린',
            (Onset::M, Nucleus::A, Coda::Null) => '마',
            (Onset::M, Nucleus::E, Coda::Null) => '머',
            (Onset::M, Nucleus::O, Coda::Null) => '모',
            (Onset::M, Nucleus::U, Coda::Null) => '무',
            (Onset::M, Nucleus::I, Coda::Null) => '미',
            (Onset::M, Nucleus::A, Coda::N) => '만',
            (Onset::M, Nucleus::E, Coda::N) => '먼',
            (Onset::M, Nucleus::O, Coda::N) => '몬',
            (Onset::M, Nucleus::U, Coda::N) => '문',
            (Onset::M, Nucleus::I, Coda::N) => '민',
            (Onset::P, Nucleus::A, Coda::Null) => '바',
            (Onset::P, Nucleus::E, Coda::Null) => '버',
            (Onset::P, Nucleus::O, Coda::Null) => '보',
            (Onset::P, Nucleus::U, Coda::Null) => '부',
            (Onset::P, Nucleus::I, Coda::Null) => '비',
            (Onset::P, Nucleus::A, Coda::N) => '반',
            (Onset::P, Nucleus::E, Coda::N) => '번',
            (Onset::P, Nucleus::O, Coda::N) => '본',
            (Onset::P, Nucleus::U, Coda::N) => '분',
            (Onset::P, Nucleus::I, Coda::N) => '빈',
            (Onset::S, Nucleus::A, Coda::Null) => '사',
            (Onset::S, Nucleus::E, Coda::Null) => '서',
            (Onset::S, Nucleus::O, Coda::Null) => '소',
            (Onset::S, Nucleus::U, Coda::Null) => '수',
            (Onset::S, Nucleus::I, Coda::Null) => '시',
            (Onset::S, Nucleus::A, Coda::N) => '산',
            (Onset::S, Nucleus::E, Coda::N) => '선',
            (Onset::S, Nucleus::O, Coda::N) => '손',
            (Onset::S, Nucleus::U, Coda::N) => '순',
            (Onset::S, Nucleus::I, Coda::N) => '신',
        }
    }
}

impl Onset {
    fn random(rng: &mut impl Rng) -> Onset {
        *[
            Onset::P,
            Onset::T,
            Onset::K,
            Onset::S,
            Onset::M,
            Onset::N,
            Onset::L,
            Onset::J,
            Onset::W,
        ]
        .choose_weighted(rng, |v| match v {
            Onset::P => 61,
            Onset::T => 45,
            Onset::K => 91,
            Onset::S => 64,
            Onset::M => 50,
            Onset::N => 32,
            Onset::L => 83,
            Onset::J => 35,
            Onset::W => 34,
            Onset::Null => 0,
        })
        .unwrap()
    }

    fn len(&self) -> usize {
        match self {
            Onset::Null => 0,
            _ => 1,
        }
    }

    fn latin_char(&self) -> Option<char> {
        match self {
            Onset::Null => None,
            Onset::P => Some('p'),
            Onset::T => Some('t'),
            Onset::K => Some('k'),
            Onset::S => Some('s'),
            Onset::M => Some('m'),
            Onset::N => Some('n'),
            Onset::L => Some('l'),
            Onset::J => Some('j'),
            Onset::W => Some('w'),
        }
    }

    fn cyrillic_char(&self) -> Option<char> {
        match self {
            Onset::Null => None,
            Onset::P => Some('п'),
            Onset::T => Some('т'),
            Onset::K => Some('к'),
            Onset::S => Some('с'),
            Onset::M => Some('м'),
            Onset::N => Some('н'),
            Onset::L => Some('л'),
            Onset::J => unreachable!(),
            Onset::W => Some('в'),
        }
    }

    fn greek_char(&self) -> Option<char> {
        match self {
            Onset::Null => None,
            Onset::P => Some('π'),
            Onset::T => Some('τ'),
            Onset::K => Some('κ'),
            Onset::S => Some('σ'),
            Onset::M => Some('μ'),
            Onset::N => Some('ν'),
            Onset::L => Some('λ'),
            Onset::J => Some('γ'),
            Onset::W => Some('β'),
        }
    }

    fn hebrew_char(&self) -> char {
        match self {
            Onset::Null => 'א',
            Onset::P => 'פ',
            Onset::T => 'ט',
            Onset::K => 'ק',
            Onset::S => 'ס',
            Onset::M => 'מ',
            Onset::N => 'נ',
            Onset::L => 'ל',
            Onset::J => 'י',
            Onset::W => 'ו',
        }
    }

    fn devanagari_char(&self) -> Option<char> {
        match self {
            Onset::Null => None,
            Onset::P => Some('प'),
            Onset::T => Some('त'),
            Onset::K => Some('क'),
            Onset::S => Some('स'),
            Onset::M => Some('म'),
            Onset::N => Some('न'),
            Onset::L => Some('ल'),
            Onset::J => Some('य'),
            Onset::W => Some('व'),
        }
    }

    fn arabic_char(&self) -> char {
        match self {
            Onset::Null => 'ا',
            Onset::P => 'ب',
            Onset::T => 'ت',
            Onset::K => 'ك',
            Onset::S => 'س',
            Onset::M => 'م',
            Onset::N => 'ن',
            Onset::L => 'ل',
            Onset::J => 'ي',
            Onset::W => 'و',
        }
    }
}

impl Nucleus {
    fn random(rng: &mut impl Rng, onset: Onset) -> Nucleus {
        *[Nucleus::A, Nucleus::E, Nucleus::I, Nucleus::O, Nucleus::U]
            .choose_weighted(rng, |v| match v {
                Nucleus::A => 146,
                Nucleus::E => 94,
                Nucleus::I if onset != Onset::T && onset != Onset::J => 140,
                Nucleus::O if onset != Onset::W => 88,
                Nucleus::U if onset != Onset::W => 64,
                _ => 0,
            })
            .unwrap()
    }

    fn len(&self) -> usize {
        1
    }

    fn latin_char(&self) -> char {
        match self {
            Nucleus::A => 'a',
            Nucleus::E => 'e',
            Nucleus::I => 'i',
            Nucleus::O => 'o',
            Nucleus::U => 'u',
        }
    }

    fn cyrillic_char(&self) -> char {
        match self {
            Nucleus::A => 'a',
            Nucleus::E => 'э',
            Nucleus::I => 'и',
            Nucleus::O => 'о',
            Nucleus::U => 'у',
        }
    }

    fn iotated_cyrillic_char(&self) -> char {
        match self {
            Nucleus::A => 'я',
            Nucleus::E => 'е',
            Nucleus::I => unreachable!(),
            Nucleus::O => 'ё',
            Nucleus::U => 'ю',
        }
    }

    fn greek_char(&self) -> char {
        match self {
            Nucleus::A => 'α',
            Nucleus::E => 'ε',
            Nucleus::I => 'ι',
            Nucleus::O => 'ο',
            Nucleus::U => 'υ',
        }
    }

    fn hebrew_char(&self) -> char {
        match self {
            Nucleus::A => 'ָ',
            Nucleus::E => 'ֶ',
            Nucleus::I => 'ִ',
            Nucleus::O => 'ֹ',
            Nucleus::U => 'ֻ',
        }
    }

    fn devanagari_char(&self) -> char {
        match self {
            Nucleus::A => 'अ',
            Nucleus::E => 'ए',
            Nucleus::I => 'इ',
            Nucleus::O => 'ओ',
            Nucleus::U => 'उ',
        }
    }

    fn devanagari_modifier_char(&self) -> Option<char> {
        match self {
            Nucleus::A => None,
            Nucleus::E => Some('े'),
            Nucleus::I => Some('ि'),
            Nucleus::O => Some('ो'),
            Nucleus::U => Some('ु'),
        }
    }

    fn arabic_chars(&self, coda: Coda) -> impl Iterator<Item = char> {
        match (self, coda) {
            (Nucleus::A, Coda::Null) => "\u{064e}".chars(),
            (Nucleus::A, Coda::N) => "\u{064b}".chars(),
            (Nucleus::E, Coda::Null) => "\u{0650}".chars(),
            (Nucleus::E, Coda::N) => "\u{064d}".chars(),
            (Nucleus::I, Coda::Null) => "\u{0650}ي".chars(),
            (Nucleus::I, Coda::N) => "\u{064d}ي".chars(),
            (Nucleus::O, Coda::Null) => "\u{064f}".chars(),
            (Nucleus::O, Coda::N) => "\u{064c}".chars(),
            (Nucleus::U, Coda::Null) => "\u{0650}و".chars(),
            (Nucleus::U, Coda::N) => "\u{064c}و".chars(),
        }
    }
}

impl Coda {
    fn random(rng: &mut impl Rng) -> Coda {
        if rng.random_bool(0.1) {
            Coda::N
        } else {
            Coda::Null
        }
    }

    fn len(&self) -> usize {
        match self {
            Coda::Null => 0,
            Coda::N => 2,
        }
    }

    fn latin_char(&self) -> Option<char> {
        match self {
            Coda::Null => None,
            Coda::N => Some('n'),
        }
    }

    fn cyrillic_char(&self) -> Option<char> {
        match self {
            Coda::Null => None,
            Coda::N => Some('н'),
        }
    }

    fn greek_char(&self) -> Option<char> {
        match self {
            Coda::Null => None,
            Coda::N => Some('ν'),
        }
    }

    fn hebrew_char(&self, last: bool) -> Option<char> {
        match self {
            Coda::Null => None,
            Coda::N => {
                if last {
                    Some('ן')
                } else {
                    Some('נ')
                }
            }
        }
    }

    fn devanagari_char(&self) -> Option<char> {
        match self {
            Coda::Null => None,
            Coda::N => Some('ं'),
        }
    }
}
