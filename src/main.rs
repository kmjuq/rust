use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("猜数字游戏");
    loop {
        println!("请输入一个数字");
        let mut guess = String::from("");
        io::stdin().read_line(&mut guess).expect("error guess");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_num) {
            Ordering::Less => println!("less"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
            Ordering::Greater => {
                println!("greater");
            }
        }
    }
    println!("bingo");
}
