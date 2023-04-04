// 模式匹配
#[cfg(test)]
mod tests {

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // 模式匹配（match）：
        // 1.允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码
        // 2.对于 if，表达式必须返回一个布尔值，而这里它可以是任何类型的
        // 3.当 match 表达式执行时，它将结果值按顺序与每一个分支的模式相比较。
        //   如果模式匹配了这个值，这个模式相关联的代码将被执行。如果模式并不匹配这个值，将继续执行下一个分支
        // 4.每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        // Rust 中的匹配是穷举式的（exhaustive）：必须穷举到最后的可能性来使代码有效
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    fn add_fancy_hat() {}

    fn remove_fancy_hat() {}

    fn move_player(num_spaces: u8) {
        println!("num_spaces = {}", num_spaces)
    }

    fn reroll() {}

    #[test]
    fn match_test() {
        // 模式匹配
        let penny = value_in_cents(Coin::Penny);
        let nickel = value_in_cents(Coin::Nickel);
        let dime = value_in_cents(Coin::Dime);
        println!("penny = {}, nickel = {}, dime = {}", penny, nickel, dime);

        // 绑定值的模式
        let alabama = value_in_cents(Coin::Quarter(UsState::Alabama));
        let alaska = value_in_cents(Coin::Quarter(UsState::Alaska));
        println!("alabama = {}, alaska = {}", alabama, alaska);

        // 匹配 Option<T>
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("five = {:?}, six = {:?}, none = {:?}", five, six, none);

        // 通配模式和 _ 占位符
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            // 最后一个分支则涵盖了所有其他可能的值，这种通配模式满足了 match 必须被穷尽的要求
            other => move_player(other),
        }

        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            // 当不想使用通配模式获取的值时，请使用 _ ，这是一个特殊的模式，可以匹配任意值而不绑定到该值
            _ => reroll(),
        }

        // if let 简单控制流
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            // 我们想要对 Some(3) 匹配进行操作但是不想处理任何其他 Some<u8> 值或 None 值。
            // 为了满足 match 表达式（穷尽性）的要求，必须在处理完这唯一的成员后加上 _ => ()
            _ => (),
        }
        // 通过简短的if let代替上面的样板代码，但这样会失去失去穷尽性检查
        if let Some(3) = some_u8_value {
            println!("three");
        }

        // let mut count = 0;
        // match coin {
        //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        //     _ => count += 1,
        // }
        // if let Coin::Quarter(state) = coin {
        //     println!("State quarter from {:?}!", state);
        // } else {
        //     count += 1;
        // }
    }
}
