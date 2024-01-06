use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(r#"       welcome to...      "#);
    println!(r#"                 _        "#);
    println!(r#"  _ __ _   _ ___| |_      "#);
    println!(r#" | '__| | | / __| __|     "#);
    println!(r#" | |  | |_| \__ \ |_      "#);
    println!(r#" |_|   \__,_|___/\__|     "#);
    println!();

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    fn takes_ownership(some_str: String) {
        println!("{}", some_str);
    }

    fn makes_copy(some_int: i32) {
        println!("{}", some_int);
    }

    let s1 = g_o();
    println!("{}", s1);

    fn g_o() -> String {
        let s_s = String::from("hellooo");
        s_s
    }

    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}", s2);

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
