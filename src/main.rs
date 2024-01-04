fn main() {
    sum();

    loop {
        println!("Guess the no : ");

        let mut g = String::new();
        io::stdin().read_line(&mut g).expect("failed");
        println!("you gussed {}", g);
    }
}

fn sum() {
    let a = 3;
    let b = 4;
    let c = a + b;
    println!("{}+{}={}", a, b, c)
}
