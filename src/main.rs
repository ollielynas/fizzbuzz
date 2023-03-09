use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::LineWriter;
use std::process::Command;
use std::thread;
use std::time::Instant;

fn main() {
    let mut input = String::new();
    println!("number of numbers (recommended range 1-9): 10^");
    io::stdin().read_line(&mut input);
    let trimmed = input.trim();

    let start = Instant::now();
    let block_size = 60;
    let total = (10_u64.pow(match trimmed.parse::<u32>() {
        Ok(a) => a,
        _ => 5,
    })) / block_size;
    let mut n = 0;
    let num_threads = 50;

    //     let num_threads = 1;
    // let total = 10_u64.pow(10);
    let inc = total / num_threads;
    let mut threads = vec![];

    let mut title = 0;
    while n < total {
        title += 1;
        threads.push(thread::spawn(move || {
            let start = n + 0;
            let file = File::create(format!(
                "{}{title} {start}.txt",
                match title {
                    _ if title < 10 => "00",
                    _ if title < 100 => "0",
                    _ => "",
                }
            ))
            .unwrap();

            let mut file = LineWriter::new(file);
            while n < start + inc {
                let a = n * block_size;
                file.write(
                    format!(
        "\nFizzBuzz\n{}\n{}\nFizz\n{}\nBuzz\nFizzBuzz\n{}\n{}\nFizz\nFizz\n{}\nFizz\n{}\n{}\nFizzBuzz\n{}\n{}\nFizz\n{}\nBuzz\nFizzBuzz\n{}\n{}\nFizz\nFizz\n{}\nFizz\n{}\n{}\nFizzBuzz\n{}\n{}\nFizz\n{}\nBuzz\nFizzBuzz\n{}\n{}\nFizz\nFizz\n{}\nFizz\n{}\n{}\nFizzBuzz\n{}\n{}\nFizz\n{}\nBuzz\nFizzBuzz\n{}\n{}\nFizz\nFizz\n{}\nFizz\n{}\n{}\n", // block size refers to the number of numbers in this solution
        a + 1,
        a + 2,
        a + 4,
        a + 7,
        a + 8,
        a + 11,
        a + 13,
        a + 14,
        a + 16,
        a + 17,
        a + 19,
        a + 22,
        a + 23,
        a + 26,
        a + 28,
        a + 29,
        a + 31,
        a + 32,
        a + 34,
        a + 37,
        a + 38,
        a + 41,
        a + 43,
        a + 44,
        a + 46,
        a + 47,
        a + 49,
        a + 52,
        a + 53,
        a + 56,
        a + 58,
        a + 59
    )
                    .as_bytes(),
                );
                n += 1;
            }
            // file.flush();
        }));
        n += inc;
    }

    for i in threads {
        i.join();
    }

    let total_sec = start.elapsed().as_secs_f64();
    println!(
        "{:+e} calculated {total_sec}sec total {:+e}per sec \n clean up remaining files ? [y/n]",
        n * block_size,
        (n * block_size) as f64 / total_sec
    );

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    if input != "n" {
        println!("combining");
        Command::new("cmd")
            .args(&["/C", "del *.log&&copy /b *.txt output.log&&del *.txt"])
            .status()
            .expect("failed to execute process");
    }
}
