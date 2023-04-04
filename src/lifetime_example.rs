// 生命周期
#[cfg(test)]
mod tests {

    use std::fmt::Display;

    // 下面这段编译无法通过：返回值需要一个泛型生命周期参数，因为 Rust 并不知道将要返回的引用是指向 x 或 y
    // 运行时无法得知if else谁会被执行，也不知道传入的引用的具体生命周期，借用检查器自身同样也无法确定，因为它不知道 x 和 y 的生命周期是如何与返回值的生命周期相关联的
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // 函数签名中的生命周期标注：
    // 单个生命周期标注本身没有多少意义，因为生命周期标注告诉 Rust 多个引用的泛型生命周期参数如何相互联系的
    // 例如如果函数有一个生命周期 'a 的 i32 的引用的参数 first。还有另一个同样是生命周期 'a 的 i32 的引用的参数 second。这两个生命周期标注意味着引用 first 和 second 必须与这泛型生命周期存在得一样久
    // 为了解决上述的错误，增加泛型生命周期参数来定义引用间的关系以便借用检查器可以进行分析，这里是想要告诉 Rust 关于参数中的引用和返回值之间的限制是他们都必须拥有相同的生命周期
    // 我们通过生命周期参数告诉 Rust 的是： longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // 如果longest函数总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期
    // 因为 y 的生命周期与参数 x 和返回值的生命周期没有任何关系
    fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
        println!("longest2 y = {}", y);
        x
    }

    // 以下代码无法编译通过，因为该函数尝试返回一个内部创建值的引用result，但result在离开函数作用域时会被清理，会引发悬垂指针问题
    // 在这种情况，最好的解决方案是返回一个有所有权的数据类型而不是一个引用，这样函数调用者就需要负责清理这个值了
    // 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配
    // 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域
    // fn longest3<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }

    // 结构体定义中的生命周期标注：定义一个存放引用的结构体，所以其定义需要生命周期标注
    // 类似于泛型参数类型，必须在结构体名称后面的尖括号中声明泛型生命周期参数，以便在结构体定义中使用生命周期参数。
    // 这个标注意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // 方法定义中的生命周期标注
    impl<'a> ImportantExcerpt<'a> {
        // impl 之后和类型名称之后的生命周期参数是必要的，不过因为第一条生命周期规则我们并不必须标注 self 引用的生命周期
        fn level(&self) -> i32 {
            3
        }

        // 这里有两个输入生命周期，所以 Rust 应用第一条生命周期省略规则并给予 &self 和 announcement 他们各自的生命周期。
        // 接着，因为其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期，这样所有的生命周期都被计算出来了
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // 结合泛型类型参数、trait bounds 和生命周期
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    #[test]
    fn lifetime_test() {
        // 生命周期的主要目标是避免悬垂引用，它会导致程序引用了非预期引用的数据
        // 下面的代码会编译报错，x引用的生命周期要小于r的生命周期，会引发悬垂指针问题
        // 将 r 的生命周期标记为 'a 并将 x 的生命周期标记为 'b，内部的 'b 块要比外部的生命周期 'a 小得多
        // 在编译时，Rust 比较这两个生命周期的大小，并发现 r 拥有生命周期 'a，不过它引用了一个拥有生命周期 'b 的对象
        // 程序被拒绝编译，因为生命周期 'b 比生命周期 'a 要小：被引用的对象比它的引用者存在的时间更短
        // {
        //     let r;                // ---------+-- 'a
        //                           //          |
        //     {                     //          |
        //         let x = 5;        // -+-- 'b  |
        //         r = &x;           //  |       |
        //     }                     // -+       |
        //                           //          |
        //     println!("r: {}", r); //          |
        // }                         // ---------+

        // 借用检查器（borrow checker），它比较作用域来确保所有的借用都是有效的
        // 下面的代码编译会通过，这里 x 拥有生命周期 'b，比 'a 要大
        // 这就意味着 r 可以引用 x：Rust 知道 r 中的引用在 x 有效的时候也总是有效的
        // {
        //     let x = 5;            // ----------+-- 'b
        //                           //           |
        //     let r = &x;           // --+-- 'a  |
        //                           //   |       |
        //     println!("r: {}", r); //   |       |
        //                           // --+       |
        // }                         // ----------+

        // 通过拥有不同的具体生命周期的 String 值调用 longest 函数
        // string1 直到外部作用域结束都是有效的，string2 则在内部作用域中是有效的，而 result 则引用了一些直到内部作用域结束都是有效的值
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }

        // 尝试在 string2 离开作用域之后使用 result
        // 以下的代码无法通过编译，string2的生命周期会返回给result，但由于超过其作用域被释放，所以不符合泛型生命周期'a的规则
        // let string1 = String::from("long string is long");
        // let result;
        // {
        //     let string2 = String::from("xyz");
        //     result = longest(string1.as_str(), string2.as_str());
        // }
        // println!("The longest string is {}", result);

        longest2(string1.as_str(), string1.as_str());

        // novel 的数据在 ImportantExcerpt 实例创建之前就存在
        // 另外，直到 ImportantExcerpt 离开作用域之后 novel 都不会离开作用域，所以 ImportantExcerpt 实例中的引用是有效的
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let ie = ImportantExcerpt {
            part: first_sentence,
        };
        println!("ImportantExcerpt = {:?}, ie.part = {}", ie, ie.part);

        println!(
            "level = {}, announce_and_return_part = {}",
            ie.level(),
            ie.announce_and_return_part(novel.as_str())
        );

        // 静态生命周期：
        // 1. 其生命周期能够存活于整个程序期间。所有的字符串字面量都拥有 'static 生命周期
        // 2. 这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。因此所有的字符串字面量都是 'static 的
        let static_str: &'static str = "I have a static lifetime.";
        println!("static_str = {}", static_str);

        let result = longest_with_an_announcement(first_sentence, novel.as_str(), 233);
        println!("longest_with_an_announcement = {}", result);
    }
}
