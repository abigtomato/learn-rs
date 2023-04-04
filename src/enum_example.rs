// 枚举
#[cfg(test)]
mod tests {

    // 该属性用于隐藏对未使用代码的警告
    #![allow(dead_code)]

    use List::*;

    // 枚举类型（enumeration）
    #[derive(Debug)]
    enum IpAddrKind {
        // 成员（variants）
        V4,
        V6,
    }

    // 具有类型的枚举
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // 内嵌了多种多样的类型
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        // 枚举方法
        fn call(&self) {
            println!("Message call {:?}", self);
        }
    }

    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // 从 `enum` 里解构出 `c`。
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // 把 `Click` 解构给 `x` and `y`。
            WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
        }
    }

    #[derive(Debug)]
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    // 通过其别名引用每个枚举变量
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    #[test]
    fn enums_test() {
        // 枚举的成员位于其标识符的命名空间中，并使用两个冒号分开
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        println!("four = {:?}, six = {:?}", four, six);

        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
        println!("home = {:?}, loopback = {:?}", home, loopback);

        // 多种类型枚举的初始化
        let msg_quit = Message::Quit;
        let msg_move = Message::Move { x: 12, y: 32 };
        let msg_write = Message::Write(String::from("rust"));
        let msg_cc = Message::ChangeColor(1, 2, 3);
        println!(
            "Quit = {:?}, Move = {:?}, Write = {:?}, ChangeColor = {:?}",
            msg_quit, msg_move, msg_write, msg_cc
        );

        // 枚举方法调用
        msg_write.call();

        // 枚举的匹配和解构
        let pressed = WebEvent::KeyPress('x');
        // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
        let pasted = WebEvent::Paste("my text".to_owned());
        let click = WebEvent::Click { x: 20, y: 80 };
        let load = WebEvent::PageLoad;
        let unload = WebEvent::PageUnload;
        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);

        // 我们可以通过别名引用每个枚举变量，避免使用又长又不方便的枚举名字
        // 最常见的情况就是在 impl 块中使用 Self 别名
        let add = Operations::Add;
        println!("Operations::Add = {:?}", add);
    }

    #[test]
    fn option_test() {
        // Option<T> 枚举：
        // 1. 包含在了 prelude 之中，不需要将其显式引入作用域
        // 2. 是Rust拥有的一个可以编码存在或不存在概念的枚举
        let some_number = Some(5);
        let some_string = Some("a string");
        println!(
            "some_number = {:?}, some_string = {:?}",
            some_number, some_string
        );

        // Some 和 None：
        // 1. 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型
        // 2. 当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值
        // 3. Option<T> 为什么就比空值要好？因为 Option<T> 和 T（这里 T 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 Option<T>
        let absent_number: Option<i32> = None;
        println!("absent_number = {:?}", absent_number);
    }

    // 使用枚举实现链表
    enum List {
        // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
        Cons(u32, Box<List>),
        // Nil：末结点，表明链表结束
        Nil,
    }

    impl List {
        fn new() -> List {
            // 创建一个空的 List 实例
            List::Nil
        }

        fn prepend(self, elem: u32) -> List {
            // 处理一个 List，在其头部插入新元素，并返回该 List
            List::Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                // 不能得到 tail 的所有权，因为 `self` 是借用的，因此使用一个对 tail 的引用 ref
                // 这里递归调用 len 方法直到遍历一遍链表
                List::Cons(_, ref tail) => 1 + tail.len(),
                // 递归的基准情形（base case）：一个长度为 0 的空列表
                Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match *self {
                // // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串
                List::Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
                Nil => format!("Nil"),
            }
        }
    }

    #[test]
    fn linked_list_example() {
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);
        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}
