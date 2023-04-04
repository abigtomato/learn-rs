// 猜数字游戏
#[cfg(test)]
mod tests {

    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    #[test]
    fn guessing_game() {
        println!("Guess the number!");

        // 生成一个1～100的随机数
        let secret_number = rand::thread_rng().gen_range(1..101);
        println!("The secret number is: {}", secret_number);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            // 从标准输入中读取数据
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {}", guess);

            // 模式匹配/比较大小
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}
