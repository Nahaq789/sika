use core::fmt;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

use futures::{lock::Mutex, stream::FuturesOrdered, StreamExt};
use rand::Rng;

const TARGET_WORD: &str = "しかのこのこのここしたんたんしかしかしかたん";
const WORD_COUNT: usize = 11;
const NUM_TASKS: usize = 10;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start_time = Instant::now();
    let cnt = Arc::new(AtomicUsize::new(0));
    let found = Arc::new(Mutex::new(false));

    let mut tasks = FuturesOrdered::new();

    for task_id in 0..NUM_TASKS {
        let task_cnt = Arc::clone(&cnt);
        let task_found = Arc::clone(&found);
        let task = tokio::spawn(async move {
            let mut local_cnt = 0;
            loop {
                local_cnt += 1;

                if *task_found.lock().await {
                    break;
                }

                task_cnt.fetch_add(1, Ordering::Relaxed);

                let result = gen_word();
                if &result == TARGET_WORD {
                    task_cnt.fetch_add(local_cnt, Ordering::Relaxed);
                    let total = task_cnt.load(Ordering::Relaxed);

                    let mut found_lock = task_found.lock().await;
                    *found_lock = true;

                    println!("found task id: {}, {}", task_id, result);
                    println!("count: {}", total);
                    println!("result time: {:?}", start_time.elapsed());
                    break;
                }
            }
        });

        tasks.push_back(task);
    }

    while let Some(item) = tasks.next().await {
        let () = item?;
    }

    Ok(())
}

fn gen_word() -> String {
    let mut rng = rand::thread_rng();
    let mut words = Vec::with_capacity(WORD_COUNT);

    for i in 0..WORD_COUNT {
        if i == 70000 {
            // words.push(" ".to_owned());
        } else {
            let result = match rng.gen_range(1..=100) {
                1..=25 => Word::Sika.to_string(),
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
    Sika,
    Noko,
    Kosi,
    Tan,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sika => write!(f, "しか"),
            Self::Noko => write!(f, "のこ"),
            Self::Kosi => write!(f, "こし"),
            Self::Tan => write!(f, "たん"),
        }
    }
}
