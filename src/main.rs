use std::io;
use rand::Rng;

fn main() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("{}",random_num);
    println!("请输入一个数字");
    let mut guess = String::from("");
    io::stdin().read_line(&mut guess).expect("error guess");
    println!("your guess {}",guess);
}