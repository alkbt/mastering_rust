use bloom::BloomFilter;
use rand::Rng;
use std::sync::{Arc, Mutex};
use rand::prelude::ThreadRng;

pub type GeneratorPtr = Arc<Mutex<Generator>>;

const LETTERS_COUNT: usize = 2;
const LETTERS_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const DIGITS_COUNT: usize = 3;
const DIGITS_CHARSET: &[u8] = b"1234567890";

lazy_static! {

    static ref TOTAL_NAMES_COUNT: u32 = LETTERS_CHARSET.len().pow(LETTERS_COUNT as u32) as u32
        + DIGITS_CHARSET.len().pow(DIGITS_COUNT as u32) as u32;
}


pub struct Generator {
    bloom: BloomFilter,
    rng: ThreadRng,
}

impl Generator {
    pub fn new() -> GeneratorPtr {
        Arc::new(Mutex::new(Generator {
            bloom: BloomFilter::with_rate(0.01, *TOTAL_NAMES_COUNT),
            rng: rand::thread_rng(),
        }))
    }

    pub fn generate(&mut self) -> Result<String, String> {
        for _ in 0..100 {
            let name = self.rand_name();
            if !self.bloom.contains(&name) {
                self.bloom.insert(&name);
                return Ok(name);
            }

            continue;
        }

        Err("name pool is exceeded".to_string())
    }

    fn rand_name(&mut self) -> String {
        self.rand_str_chs(LETTERS_CHARSET, LETTERS_COUNT)
            + self.rand_str_chs(DIGITS_CHARSET, DIGITS_COUNT).as_str()
    }

    fn rand_str_chs(&mut self, charset: &[u8], len: usize) -> String {
        (0..len)
            .map(|_| {
                let idx = self.rng.gen_range(0, charset.len());
                charset[idx] as char
            })
            .collect()
    }
}

