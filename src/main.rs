use core::fmt;
use std::time::Instant;

use rand::Rng;

const TARGET_WORD: &str = "しかのこのこのここしたんたん しかのこのこのここしたんたん";
const WORD_COUNT: usize = 15;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start_time = Instant::now();
    let mut cnt = 0;

    loop {
        cnt += 1;
        let result = gen_word();
        // println!("{}", result);
        if &result == TARGET_WORD {
            println!("{}", result);
            println!("count: {}", cnt);
            println!("result time: {:?}", start_time.elapsed());
            break;
        }
    }

    Ok(())
}

fn gen_word() -> String {
    let mut rng = rand::thread_rng();
    let mut words = Vec::with_capacity(WORD_COUNT);

    for i in 0..WORD_COUNT {
        if i == 7 {
            words.push(" ".to_owned());
        } else {
            let result = match rng.gen_range(1..=100) {
                1..=25 => Word::SIka.to_string(),
                26..=50 => Word::Noko.to_string(),
                51..=75 => Word::Kosi.to_string(),
                76..=100 => Word::Tan.to_string(),
                _ => unreachable!("Error"),
            };
            words.push(result);
        }
    }
    words.iter().map(|c| c.to_string()).collect()
}

#[derive(Debug)]
enum Word {
    SIka,
    Noko,
    Kosi,
    Tan,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SIka => write!(f, "しか"),
            Self::Noko => write!(f, "のこ"),
            Self::Kosi => write!(f, "こし"),
            Self::Tan => write!(f, "たん"),
        }
    }
}
