use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::process::Command;
use std::thread;
use std::time::Instant;
use std::io;
use std::io::*;

fn fizz(i: u32) -> String {
    let a = i * 15;
    return format!(
        r"FizzBuzz\n{}\n{}\Fizz\n{}\nBuzz\nFizzBuzz\n{}\n{}\nFizz\nBuzz\n{}\nFizz\n{}\n{}\n",
        a + 1,
        a + 2,
        a + 4,
        a + 7,
        a + 8,
        a + 11,
        a + 13,
        a + 14
    );
}

fn main() {
    let start = Instant::now();
    let total = (10_u64.pow(5)) / 15;
    let mut n = 0;
    let num_threads = 150;
    //     let num_threads = 1;
    // let total = 10_u64.pow(10);
    let inc = total / num_threads;
    let mut threads = vec![];

    let mut title = 0;
    while n < total {
        title += 1;
        threads.push(thread::spawn(move || {
            let start = n + 0;
            let file = File::create(format!("{title} {start}.txt")).unwrap();

            let mut file = LineWriter::new(file);
            while n < start + inc {
                file.write(fizz(n as u32).as_bytes());
                n += 1;
            }
            file.flush();
        }));
        n += inc;
    }

    for i in threads {
        i.join();
    }
    let total_sec = start.elapsed().as_secs_f64();
    println!(
        "{:+e} calculated {total_sec}sec total {:+e}per sec \n clean up remaining files ? [y/n]",
        n * 15,
        (n * 15) as f64 / total_sec
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
