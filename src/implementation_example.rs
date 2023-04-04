// 方法
#[cfg(test)]
mod tests {

    // 必须为结构体显式选择打印出调试信息的功能，在结构体定义之前加上外部属性 #[derive(Debug)]
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // 实现 impl 块（impl 是 implementation 的缩写）
    // 1.这个 impl 块中的所有内容都将与 Rectangle 类型相关联，每个结构体都允许拥有多个 impl 块
    // 2.在一个 impl 块中，Self 类型是 impl 块的类型的别名，在 self 前面使用 & 来表示这个方法借用了 Self 实例
    // 3.如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self
    impl Rectangle {
        // 1.方法与函数类似：使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
        // 2.不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文）
        // 3.方法的第一个参数总是 self，它代表调用该方法的结构体实例
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn width(&self) -> bool {
            self.width > 0
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // 关联函数（associated function）
        // 1.所有在 impl 块中定义的函数
        // 2.只与impl后的类型相关，不与实例相关
        // 3.关联函数经常被用作返回一个结构体新实例的构造函数
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    // 仅仅使用结构体的数据而不需要获取所有权
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    #[test]
    fn implementation_test() {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        // 函数调用
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );

        // :? 指示符告诉 println! 我们想要使用叫做 Debug 的输出格式。Debug 是一个 trait，它允许我们以一种对开发者有帮助的方式打印结构体
        println!("rect1 is {:?}", rect1);
        println!("rect1 is {:#?}", rect1);

        // dbg! 宏接收一个表达式的所有权，打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权
        dbg!(&rect1);

        // 方法调用
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );

        // 自动引用和解引用（automatic referencing and dereferencing）
        // 1.当使用 rect1.width() 调用方法时，Rust 会自动为 rect1 添加 &、&mut 或 * 以便使 rect1 与方法签名（&self）匹配
        // 2.即 rect1.width() 和 (&rect1).width() 是等价的
        // 3.因为方法有一个明确的接收者 self 的类型。在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）
        if rect1.width() {
            println!("The rectangle has a nonzero width; it is {}", rect1.width);
        }

        // 带有更多参数的方法
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

        // 关联函数调用：这个函数位于结构体的命名空间中，:: 语法用于关联函数和模块创建的命名空间
        let sq = Rectangle::square(3);
        println!("sq = {:?}", sq);
    }
}
