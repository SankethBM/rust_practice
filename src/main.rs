use colored::Colorize;
use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn b_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("Sanketh"),
        email: String::from("sankethbm@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("Sanketh B M");

    println!("{:#?}", user1);

    let user2 = b_user(String::from("karthik@gmail.com"), String::from("karthik"));
    println!("{:#?}", user2);

    let user3 = User {
        email: String::from("value@example.com"),
        username: String::from("value"),
        ..user2
    };
    println!("{:#?}", user3);

    let user4 = User { ..user1 };
    println!("{:#?}", user4);

    let user5 = User {
        email: user3.email,
        username: user2.username,
        ..user3
    };
    println!("{:#?}", user5);

    #[derive(Debug)]
    struct Color(i32, i32, i32); //<----------------Tuple Structs------------------>
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let b = Color(0, 0, 0);
    let a = Point(1, 2, 3);

    println!("{:#?} {:#?}", a, b); //<----------------//Tuple Structs------------------>

    //finding area of rectangle  using tuple struct
    //  1   //
    let w = 30;
    let h = 50;

    println!("Area of rectangle is {}", area(w, h));

    fn area(w: u32, h: u32) -> u32 {
        w * h
    }

    //  2   //
    let rect: (u32, u32) = (25, 30);

    println!("Area of Rect is {}", area1(rect));

    fn area1(dimentions: (u32, u32)) -> u32 {
        dimentions.0 * dimentions.1
    }

    //  3   //
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 4,
        height: 9,
    };
    println!("Rectangle Area : {}", area2(&rect1));

    fn area2(rec: &Rectangle) -> u32 {
        rec.width * rec.height
    }

    //  4 using methods //
    impl Rectangle {
        fn are(&self) -> u32 {
            self.width * self.height
        }
    }
    let re = Rectangle {
        width: 30,
        height: 30,
    };

    println!("{:#?}", re.are())

    // <-------------------------------------------------------------------  //structs  -------------------------------------------------------------------->

    // let numbers = (1, 2, 3);
    // let second = numbers.1;

    // assert_eq!(2, second, "This is not the 2nd number in the tuple!")

    // let cat = ("Furry McFurson", 3.5);
    // let (name, age) = cat;

    // println!("{} is {} years old.", name, age);

    // slice_out_of_array();

    // fn slice_out_of_array() {
    //     let a = [1, 2, 3, 4, 5];

    //     let nice_slice = &a[1..4];

    //     assert_eq!([2, 3, 4], nice_slice)
    // }

    // let a = [1; 10];

    // if a.len() >= 100 {
    //     println!("Wow, that's a big array!");
    // } else {
    //     println!("Meh, I eat arrays like that for breakfast.");
    //     panic!("Array not big enough, more elements needed")
    // }

    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[0..3];
    // println!("{:?}", slice);

    // let mut s = String::from("hello world");

    // let s2 = "hello world";

    // let word = f_w(&s2);
    // println!("{}", word);

    // fn f_w(s: &str) -> &str {
    //     let bytes = s.as_bytes();

    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return &s[0..i];
    //         }
    //     }

    //     &s[..]
    // }

    // No test changes needed!
    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     pub fn foo_if_fizz(fizzish: &str) -> &str {
    //         if fizzish == "fizz" {
    //             "foo"
    //         } else {
    //             "one"
    //         }
    //     }

    //     #[test]
    //     fn foo_for_fizz() {
    //         assert_eq!(foo_if_fizz("fizz"), "foo")
    //     }

    //     #[test]
    //     fn bar_for_fuzz() {
    //         assert_eq!(foo_if_fizz("fuzz"), "bar")
    //     }

    //     #[test]
    //     fn default_to_baz() {
    //         assert_eq!(foo_if_fizz("literally anything"), "baz")
    //     }
    // }

    // let c = bigger(5, 4);
    // println!("bigger no is {}", bigger(6, 4));

    // pub fn bigger(a: i32, b: i32) -> i32 {
    //     if a > b {
    //         a
    //     } else {
    //         b
    //     }
    // }

    // let original_price = 50;
    // println!("Your sale price is {}", sale_price(original_price));

    // fn sale_price(price: i32) -> i32 {
    //     if is_even(price) {
    //         price - 10
    //     } else {
    //         price - 3
    //     }
    // }

    // fn is_even(num: i32) -> bool {
    //     num % 2 == 0
    // }

    // let s = String::from("hello");
    // takes_ownership(s);
    // println!("{}", s);

    // let x = 5;
    // makes_copy(x);
    // println!("{}", x);

    // fn takes_ownership(some_str: String) {
    //     println!("{}", some_str);
    // }

    // fn makes_copy(some_int: i32) {
    //     println!("{}", some_int);
    // }

    // let s1 = g_o();
    // println!("{}", s1);

    // let s2 = String::from("hi");
    // println!("{} s2", s2);
    // let s3 = tagb(s2);
    // println!("{} s3", s3);

    // fn g_o() -> String {
    //     let s_s = String::from("hellooo");
    //     s_s
    // }

    // fn tagb(a_str: String) -> String {
    //     a_str
    // }

    // let x = 5;
    // let y = x;
    // println!("{} {}", x, y);

    // let s11 = String::from("hellooow");

    // let s2 = s11.clone();
    // println!("{}", s2);

    // let l = cal_len(&s11);
    // println!("length of {} is {}", s11, l);

    // let mut s12 = String::from("hello");
    // change_str(&mut s12);
    // println!("{}", s12);

    // fn cal_len(s: &String) -> usize {
    // let l = s.len();
    // l
    // s.len()
    // }

    // fn change_str(s: &mut String) {
    //     s.push_str(" world")
    // }

    // let mut a1 = String::from("Hii");

    // let r1 = &a1;
    // let r2 = &a1;

    // println!("{} r1 {} r2", r1, r2); //here the scope of immutable variable a1 will end

    // let r3 = &mut a1; //we can make access to a mutable variable outside its scope , and mut variable can only accessed once , and also 1 is immut and other is mut is not accessible
    // println!("{}", r3);

    // sum();

    // let reval = ret_val();
    // println!("return value is {}", reval);

    // loop {
    //     println!("Guess the no : ");

    //     let mut g = String::new();
    //     io::stdin().read_line(&mut g).expect("failed");
    //     println!("you gussed {}", g);

    //     let g: u32 = g.trim().parse().expect("Enter a no");

    //     let sn = rand::thread_rng().gen_range(1, 101);
    //     println!("Generated no : {}", sn);
    //     match g.cmp(&sn) {
    //         Ordering::Less => println!("{}", "small".red()),
    //         Ordering::Greater => println!("{}", "big".red()),
    //         Ordering::Equal => {
    //             println!("{}", "You win".green());
    //             break;
    //         }
    //     }
    // }
}

// fn sum() {
//     let a = 3;
//     let b = 4;
//     let c = a + b;
//     println!("{}+{}={}", a, b, c)
// }

// fn ret_val() -> i32 {
//     5
// }
