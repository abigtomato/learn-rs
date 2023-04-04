fn add_one(x: i32) -> i32 {
    x + 1
}

// 以向函数传递常规函数！这在我们希望传递已经定义的函数而不是重新定义闭包作为参数时很有用
// 通过函数指针允许我们使用函数作为另一个函数的参数。函数的类型是 fn （使用小写的 “f” ）以免与 Fn 闭包 trait 相混淆
// fn 被称为 函数指针（function pointer）。指定参数为函数指针的语法类似于闭包
// 不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数
// 函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
}

// 函数不能直接返回闭包，如这种格式 Fn(i32) -> i32，因为 Rust 并不知道需要多少空间来储存闭包
// 解决方法是使用trait对象来包装闭包类型
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

pub fn senior_fn_example() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // 可以将函数作为 map 的参数来代替闭包
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        // 这里使用了定义于 ToString trait 的 to_string 函数，标准库为所有实现了 Display 的类型实现了这个 trait
        .map(ToString::to_string)
        .collect();
    println!("list_of_strings = {:?}", list_of_strings);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("list_of_statuses = {:?}", list_of_statuses);

    // 函数返回闭包
    println!("returns_closure = {}", returns_closure()(1));
}
