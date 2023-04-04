// 模块
#[cfg(test)]
mod tests {

    /*
        crate（root）
        └── mod_example
            └── tests
                |── front_of_house
                |    ├── hosting
                |    │      ├── add_to_waitlist
                |    │      └── seat_at_table
                |    └── serving
                |           ├── take_order
                |           ├── serve_order
                |           └── take_payment
                └── eat_at_restaurant
    */

    // 关键字 mod 定义一个模块
    mod front_of_house {
        // 模块中包含其他模块
        // Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。
        // 父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项
        // 这是因为子模块封装并隐藏了他们的实现详情，但是子模块可以看到他们定义的上下文
        pub mod hosting {
            pub fn add_to_waitlist() {
                seat_at_table()
            }

            fn seat_at_table() {
                // 使用 super 开头来构建从父模块开始的相对路径
                super::serving::take_order();
            }
        }

        // 通过使用模块，我们可以把相关的定义组织起来，并通过模块命名来解释为什么它们之间有相关性。
        // 使用这部分代码的开发者可以更方便的循着这种分组找到自己需要的定义，而不需要通览所有
        mod serving {
            // 模块中也可以包含其他项，比如结构体、枚举、常量、trait或函数
            pub fn take_order() {
                server_order()
            }

            fn server_order() {
                take_payment()
            }

            fn take_payment() {
                println!("take_payment")
            }
        }
    }

    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }

            pub fn get_seasonal_fruit(&self) -> &str {
                &self.seasonal_fruit[..]
            }
        }

        // 共有枚举的成员默认就是公有的
        #[derive(Debug)]
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    // 使用 use 关键字将名称引入作用域，现在 hosting 在作用域中就是有效的名称了
    // 在作用域中增加 use 和路径类似于在文件系统中创建软连接（符号连接，symbolic link）
    pub use crate::mod_example::tests::front_of_house::hosting; // 绝对路径引入
                                                                // use front_of_house::hosting; // 相对路径引入

    // 嵌套路径引入
    // use std::{cmp::Ordering, dbg};
    // use std::io::{self, Write};

    // 路径用于引用模块树中的项
    #[test]
    fn eat_at_restaurant() {
        // 绝对路径（absolute path）从 crate 根部开始，以 crate 名或者字面量 crate 开头
        crate::mod_example::tests::front_of_house::hosting::add_to_waitlist();

        // 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头
        front_of_house::hosting::add_to_waitlist();

        // 调用共有结构体的共有方法
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // 共有结构体的共有字段
        meal.toast = String::from("Wheat");
        println!(
            "toast = {}, seasonal_fruit = {}",
            meal.toast,
            meal.get_seasonal_fruit()
        );

        // 共有枚举成员获取
        let soup = back_of_house::Appetizer::Soup;
        let salad = back_of_house::Appetizer::Salad;
        println!("soup = {:?}, salad = {:?}", soup, salad);

        // 使用use引入作用域后直接用名称调用
        hosting::add_to_waitlist();
    }
}
