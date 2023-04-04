// 特质
#[cfg(test)]
mod tests {

    use core::fmt::Debug;
    use std::fmt::Display;

    // 一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。
    // trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。
    pub trait Summary {
        // 定义一系列类型所共同需要的行为，只有方法签名，没有实现
        // 编译器也会确保任何实现 Summary trait 的类型都拥有与这个签名的定义完全一致的 summarize 方法
        fn summarize(&self) -> String;

        // 默认实现，当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为
        fn summarize2(&self) -> String {
            // 默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。如此，trait 可以提供很多有用的功能而只需要实现指定一小部分内容
            format!("(Read more from {}...)", self.summarize())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // 让结构体实现特性
    impl Summary for NewsArticle {
        // 实现特性的行为
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    // 结构体实现多个特性
    impl Display for NewsArticle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            println!("f.fill() = {:?}", f.fill());
            Ok(())
        }
    }

    // 结构体实现多个特性
    impl Clone for NewsArticle {
        fn clone(&self) -> Self {
            Self {
                headline: self.headline.clone(),
                location: self.location.clone(),
                author: self.author.clone(),
                content: self.content.clone(),
            }
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    // 实现 trait 时需要注意的一个限制是，只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait
    // 但是不能为外部类型实现外部 trait，这个限制是被称为 相干性（coherence） 的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // 结构体实现多个特性
    impl Clone for Tweet {
        fn clone(&self) -> Self {
            Self {
                username: self.username.clone(),
                content: self.content.clone(),
                reply: self.reply.clone(),
                retweet: self.retweet.clone(),
            }
        }
    }

    // 结构体实现多个特性
    impl Debug for Tweet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Tweet")
                .field("username", &self.username)
                .field("content", &self.content)
                .field("reply", &self.reply)
                .field("retweet", &self.retweet)
                .finish()
        }
    }

    // trait 作为参数：该参数支持任何实现了指定 trait 的类型
    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Trait Bound 特性绑定：泛型 T 被指定为 item1 和 item2 的参数限制，如此传递给参数 item1 和 item2 值的具体类型必须一致
    fn notify2<T: Summary>(item1: &T, item2: &T) {
        println!(
            "notify2: item1.summarize() = {}, item2.summarize() = {}",
            item1.summarize(),
            item2.summarize()
        );
    }

    // 通过 + 指定多个特性绑定：
    // 如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，那么 item 就需要同时实现两个不同的 trait：Display 和 Summary
    // 可以这样写 fn notify3(item: impl Summary + Display)，也可以写成下面的简写形式
    fn notify3<T: Summary + Display>(item: &T) {
        println!("notify4 item {}", item.summarize());
    }

    // 通过 where 简化 trait bound：
    // 每个泛型有其自己的 trait bound，所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，这使得函数签名难以阅读
    // 比如这样 fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32
    fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        println!("t = {}, u = {:?}", t, u);
        1
    }

    // 返回实现了 trait 的类型：
    // 指定了 returns_summarizable 函数返回某个实现了 Summary trait 的类型，但是不确定其具体的类型
    // 在闭包和迭代器场景十分的有用，闭包和迭代器创建只有编译器知道的类型，或者是非常非常长的类型。impl Trait 允许你简单的指定函数返回一个 Iterator 而无需写出实际的冗长的类型
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // 使用 trait bounds 来修复 largest 函数（查找最大值）：
    // 1. 大于运算符（>）比较两个 T 类型的值。这个运算符被定义为标准库中 trait std::cmp::PartialOrd 的一个默认方法，需要在 T 的 trait bound 中指定 PartialOrd
    // 2. i32 和 char 这样的类型是已知大小的并可以储存在栈上，所以他们实现了 Copy trait。当我们将 largest 函数改成使用泛型后，现在 list 参数的类型就有可能是没有实现 Copy trait 的。这意味着我们可能不能将 list[0] 的值移动到 largest 变量中
    // 3. 另一种 largest 的实现方式是返回在 slice 中 T 值的引用。如果我们将函数返回值从 T 改为 &T 并改变函数体使其能够返回一个引用，我们将不需要任何 Clone 或 Copy 的 trait bounds 而且也不会有任何的堆分配
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 使用 trait bound 有条件地实现方法
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        // 类型 Pair<T> 总是实现了 new 方法
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        // 只有那些为 T 类型实现了 PartialOrd trait （来允许比较） 和 Display trait （来启用打印）的 Pair<T> 才会实现 cmp_display 方法
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // trait 和 trait bound 让我们使用泛型类型参数来减少重复，并仍然能够向编译器明确指定泛型类型需要拥有哪些行为。
    // 因为我们向编译器提供了 trait bound 信息，它就可以检查代码中所用到的具体类型是否提供了正确的行为。
    // 在动态类型语言中，如果我们尝试调用一个类型并没有实现的方法，会在运行时出现错误。
    // Rust 将这些错误移动到了编译时，甚至在代码能够运行之前就强迫我们修复错误。
    // 另外，我们也无需编写运行时检查行为的代码，因为在编译时就已经检查过了，这样相比其他那些不愿放弃泛型灵活性的语言有更好的性能。
    #[test]
    fn trait_test() {
        let tweet = &Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        let article = &NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };

        notify(tweet);
        notify2(article, article);
        notify3(article);
        some_function(article, tweet);
        returns_summarizable();

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);

        let pair = Pair::new(1, 2);
        println!("pair = {:?}", pair.cmp_display());
    }
}
