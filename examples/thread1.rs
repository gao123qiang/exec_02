use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_RPODUCES: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()>{
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_RPODUCES {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit");
        11
    });

    let secret = consumer.join().map_err(|e| anyhow!("thread join error: {:?}", e))?;

    println!("secret: {}", secret);

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_name = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_name));
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self {idx, value}
    }
}