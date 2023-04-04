// 智能指针
#[cfg(test)]
mod tests {

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use crate::smart_pointers_example::tests::List::{Cons, Nil};
    use std::ops::Deref;

    // 最简单直接的智能指针是 box，其类型是 Box<T>。 box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针
    // 除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：
    // 1. 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候（box 允许创建递归类型）
    // 2. 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候（转移大量数据的所有权可能会花费很长的时间，因为数据在栈上进行了拷贝。为了改善这种情况下的性能，可以通过 box 将这些数据储存在堆上）
    // 3. 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候（ trait 对象（trait object））
    #[test]
    fn box_example() {
        // 这里定义了变量 b，其值是一个指向被分配在堆上的值 5 的 Box
        // 当像 b 这样的 box 在 main 的末尾离开作用域时，它将被释放
        // 这个释放过程作用于 box 本身（位于栈上）和它所指向的数据（位于堆上）
        let b = Box::new(5);
        println!("b = {}", b);

        // 一种无法在编译时知道大小的类型是 递归类型（recursive type），其值的一部分可以是相同类型的另一个值。这种值的嵌套理论上可以无限的进行下去，所以 Rust 不知道递归类型需要多少空间
        // 编译器尝试计算出储存一个 List 枚举需要多少内存，并开始检查 Cons 成员，那么 Cons 需要的空间等于 i32 的大小加上 List 的大小。为了计算 List 需要多少内存，它检查其成员，从 Cons 成员开始。Cons 成员储存了一个 i32 值和一个 List 值，这样的计算将无限进行下去
        // 因为 Box<T> 是一个指针，我们总是知道它需要多少空间：指针的大小并不会根据其指向的数据量而改变。这意味着可以将 Box 放入 Cons 成员中而不是直接存放另一个 List 值。Box 会指向另一个位于堆上的 List 值，而不是存放在 Cons 成员中
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("list = {:?}", list);

        // 通过解引用运算符追踪指针的值
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);

        // 像引用一样使用 Box<T>
        // 唯一不同的地方就是将 y 设置为一个指向 x 值的 box 实例，而不是指向 x 值的引用
        // 可以使用解引用运算符以 y 为引用时相同的方式追踪 box 的指针
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // 通过实现 Deref trait 将某类型像引用一样处理
    impl<T> Deref for MyBox<T> {
        // 定义了用于此 trait 的关联类型
        type Target = T;

        // 让解引用有效的关键（*运算符），deref 返回值了值的引用
        // deref 方法向编译器提供了一种能力：能够获取任何实现了 Deref trait 的类型的值
        // 并且可以通过调用这个类型的 deref 方法来获取一个解引用方法已知的 & 引用
        fn deref(&self) -> &T {
            &self.0
        }
    }

    // 当我们将特定类型的值的引用作为参数传递给函数或方法，但是被传递的值的引用与函数或方法中定义的参数类型不匹配时，会发生解引用强制转换。
    // 这时会有一系列的 deref 方法被调用，把我们提供的参数类型转换成函数或方法需要的参数类型
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    #[test]
    fn deref_trait_example() {
        // 自定义智能指针
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        // Rust 事实上在底层运行了如下代码：*(y.deref())
        // 每次当我们在代码中使用 * 时， * 运算符都被替换成了先调用 deref 方法再接着使用 * 解引用的操作，且只会发生一次，不会对 * 操作符无限递归替换
        assert_eq!(5, *y);

        // 函数和方法的隐式解引用强制转换：
        // 1. 解引用强制转换（deref coercions）是 Rust 在函数或方法传参上的一种便利。解引用强制转换只能工作在实现了 Deref trait 的类型上。解引用强制转换将一种类型（A）隐式转换为另外一种类型（B）的引用，因为 A 类型实现了 Deref trait，并且其关联类型是 B 类型
        // 2. MyBox<T> 上实现了 Deref trait，Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String，标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice，Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了
        // 3. 当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型。这些解析都发生在编译时，所以利用解引用强制转换并没有运行时损耗
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        // 和上面的代码同义
        hello(&(*m)[..]);
    }

    #[derive(Debug)]
    struct CustomSmartPointer {
        data: String,
    }

    // Drop trait 包含在 prelude 中，所以无需导入它
    // drop 析构函数（destructor）的函数体是放置任何当类型实例离开作用域时期望运行的逻辑的地方
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    // 所有权系统确保引用总是有效的，也会确保 drop 只会在值不再被使用时被调用一次
    #[test]
    fn drop_trait_example() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created. c = {:?}, d = {:?}", c, d);
        // 通过 std::mem::drop 提早丢弃值
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }

    use std::rc::Rc;

    #[derive(Debug)]
    enum List2 {
        Cons(i32, Rc<List2>),
        Nil,
    }

    // 大部分情况下所有权是非常明确的：可以准确地知道哪个变量拥有某个值。然而，有些情况单个值可能会有多个所有者
    // 为了启用多所有权，Rust 有一个叫做 Rc<T> 的类型。其名称为 引用计数（reference counting）的缩写。
    // 引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用，如果某个值有零个引用，就代表没有任何有效引用并可以被清理
    // Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候
    #[test]
    fn rc_example() {
        // 下面的代码无法通过编译：当创建b时候，a已经被移动进了b，接着通过使用a创建c时，则不被允许
        // let a = Cons(5,
        //     Box::new(Cons(10,
        //         Box::new(Nil))));
        // let b = Cons(3, Box::new(a));
        // let c = Cons(4, Box::new(a));

        // 通过给a增加引用计数的方式，让a可以被多个地方引用
        let a = Rc::new(List2::Cons(
            5,
            Rc::new(List2::Cons(10, Rc::new(List2::Nil))),
        ));
        // a 中 Rc<List> 的初始引用计数为1
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = List2::Cons(3, Rc::clone(&a));
        // 每次调用 clone，计数会增加1，此处为2
        println!(
            "count after creating b = {:?}, count = {}",
            b,
            Rc::strong_count(&a)
        );
        {
            let c = List2::Cons(4, Rc::clone(&a));
            // 计数加1，此处为3
            println!(
                "count after creating c = {:?}, count = {}",
                c,
                Rc::strong_count(&a)
            );
        }
        // c离开作用域被释放，a的引用计数减1，此处为2
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    #[test]
    fn refcell_example() {
        todo!()
    }
}
