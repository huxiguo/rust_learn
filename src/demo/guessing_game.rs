use rand::Rng;
use std::cmp::Ordering;
use std::io;
/**
 * 猜数字游戏
 */
pub fn run() {
    println!("Guess the number!");
    // 生成一个1到100之间的随机数
    let secret_number = rand::rng().random_range(1..=100);

      loop {
         println!("请输入一个数字🔢!");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! The secret number is {secret_number}");
                break;
            }
        }
    }
}
