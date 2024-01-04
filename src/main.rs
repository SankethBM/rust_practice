use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    sum();

    let reval = ret_val();
    println!("return value is {}", reval);

    loop {
        println!("Guess the no : ");

        let mut g = String::new();
        io::stdin().read_line(&mut g).expect("failed");
        println!("you gussed {}", g);

        let g: u32 = g.trim().parse().expect("Enter a no");

        let sn = rand::thread_rng().gen_range(1, 101);
        println!("Generated no : {}", sn);
        match g.cmp(&sn) {
            Ordering::Less => println!("{}", "small".red()),
            Ordering::Greater => println!("{}", "big".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
        }
    }
}

fn sum() {
    let a = 3;
    let b = 4;
    let c = a + b;
    println!("{}+{}={}", a, b, c)
}

fn ret_val() -> i32 {
    5
}
