use rand::Rng;
use std::cmp::Ordering::*;
use std::io;
use std::io::Write;

fn main() {
    let secret = rand::thread_rng()
        .gen_range(1, 101);

    loop {
        print!("guess: ");
        io::stdout()
            .flush()
            .expect("🤬 cannot flush");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("🤬 cannot read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("🤬 not a number");
                continue;
            }
        };
        println!("you say: {}, let's see...", guess);

        match guess.cmp(&secret) {
            Less => println!("🤨 bigger"),
            Greater => println!("🤨 smaller"),
            Equal => {
                println!("🥳 you rock!");
                break; // exit the loop
            }
        }
        println!();
    }
}
