// 测试模块的 #[cfg(test)] 标注告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，而在运行 cargo build 时不这么做
#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }
            Guess { value }
        }
    }

    fn prints_and_returns_10(a: i32) -> i32 {
        println!("I got the value {}", a);
        10
    }

    #[test]
    fn testing_example() {
        let rect = Rectangle {
            width: 12,
            height: 12,
        };
        println!("rect.width = {}, rect.height = {}", rect.width, rect.height);
        println!(
            "rect.can_hold = {}",
            rect.can_hold(&Rectangle {
                width: 10,
                height: 10
            })
        );
        println!("add_two = {}", add_two(100));
        println!("greeting = {}", greeting("name"));
        let guess = Guess::new(50);
        println!("guess.value = {}", guess.value);
        println!("prints_and_returns_10 = {}", prints_and_returns_10(100));
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        // 布尔断言
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    // 标记为忽略的测试
    #[ignore]
    fn it_adds_two() {
        // 相等断言
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // 自定义失败信息
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // should_panic检查代码是否按期望处理错误
    // expected匹配具体的错误信息
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        // 不同于调用 assert_eq! 宏，而是在测试通过时返回 Ok(())，在测试失败时返回带有 String 的 Err
        // 这样编写测试来返回 Result<T, E> 就可以在函数体中使用问号运算符，如此可以方便的编写任何运算符会返回 Err 成员的测试
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
}
