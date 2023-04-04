fn if_let_example() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // if let 也可以组合并匹配 if let、else if 和 else if let 表达式。
    // 这相比 match 表达式一次只能将一个值与模式比较提供了更多灵活性；
    // 一系列 if let、else if、else if let 分支并不要求其条件相互关联
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    // 引入了一个新的覆盖变量 age，它包含 Ok 成员中的值
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    // if let 表达式的缺点在于其穷尽性没有为编译器所检查，而 match 表达式则检查了。如果去掉最后的 else 块而遗漏处理一些情况，编译器也不会警告这类可能的逻辑错误
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let_example() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // while let 条件循环，它允许只要模式匹配就一直进行 while 循环
    // while 循环只要 pop 返回 Some 就会一直运行其块中的代码。一旦其返回 None，while 循环停止
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_example() {
    let v = vec!['a', 'b', 'c'];

    // for 可以获取一个模式。在 for 循环中，模式是 for 关键字直接跟随的值，正如 for x in y 中的 x
    // 使用 enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引，他们位于一个元组中。
    // 第一个 enumerate 调用会产生元组 (0, 'a')。当这个值匹配模式 (index, value)，index 将会是 0 而 value 将会是 'a'
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn let_example() {
    // let 语句更为正式的样子：let PATTERN = EXPRESSION;
    // x 是一个表示“将匹配到的值绑定到变量 x” 的模式。同时因为名称 x 是整个模式，这个模式实际上等于 “将任何值绑定到变量 x，不管值是什么”。
    let x = 5;
    println!("x = {}", x);
    // 这里将一个元组与模式匹配。Rust 会比较值 (1, 2, 3) 与模式 (x, y, z) 并发现此值匹配这个模式。
    // 在这个例子中，将会把 1 绑定到 x，2 绑定到 y 并将 3 绑定到 z。你可以将这个元组模式看作是将三个独立的变量模式结合在一起
    let (x, y, z) = (1, 2, 3);
    println!("x = {}, y = {}, z = {}", x, y, z);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeColor2(Color),
}

enum Hello {
    Hi { id: i32 },
}

// 模式是 Rust 中特殊的语法，它用来匹配类型中的结构，结合使用模式和 match 表达式以及其他结构可以提供更多对程序控制流的支配权
// 模式由如下一些内容组合而成：
// 1. 字面量
// 2. 解构的数组、枚举、结构体或者元组
// 3. 变量
// 4. 通配符
// 5. 占位符
pub fn mode_example() {
    if_let_example();
    while_let_example();
    for_example();
    let_example();

    // 可以在函数参数中匹配元组，值 &(3, 5) 会匹配模式 &(x, y)，如此 x 得到了值 3，而 y得到了值 5
    let point = (3, 5);
    print_coordinates(&point);

    // 能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）
    // 因为 x 可以匹配任何值所以不可能会失败
    let x = 5;
    println!("x = {}", x);

    // 对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）
    // 如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配
    if let Some(x) = Option::Some(123) {
        println!("x = {}", x);
    }

    // 匹配字面量
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配命名变量
    // 命名变量是匹配任何值的不可反驳模式
    // match 会开始一个新作用域，match 表达式中作为模式的一部分声明的变量会覆盖 match 结构之外的同名变量，与所有变量一样
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // 第二个匹配分支中的模式引入了一个新变量 y，它会匹配任何 Some 中的值
        // 一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 多个模式
    // 在 match 表达式中，可以使用 | 语法匹配多个模式，它代表 或（or）的意思
    // 如下代码将 x 的值与匹配分支相比较，第一个分支有 或 选项，意味着如果 x 的值匹配此分支的任一个值，它就会运行
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 通过 ..= 匹配值的范围
    // 如果 x 是 1、2、3、4 或 5，第一个分支就会匹配。这相比使用 | 运算符表达相同的意思更为方便
    // 范围只允许用于数字或 char 值，因为编译器会在编译时检查范围不为空。char 和 数字值是 Rust 仅有的可以判断范围是否为空的类型
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 解构结构体
    let p = Point { x: 0, y: 7 };
    // 创建了变量 a 和 b 来匹配结构体 p 中的 x 和 y 字段
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // 创建了变量 x 和 y，与变量 p 中的 x 和 y 相匹配。其结果是变量 x 和 y 包含结构体 p 中的值
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    // match 语句将 Point 值分成了三种情况：直接位于 x 轴上（此时 y = 0 为真）、位于 y 轴上（x = 0）或不在任何轴上的点
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 解构枚举
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        _ => (),
    }
    println!(
        "Quit = {:?}, Move = {:?}, Write = {:?}",
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("233"))
    );

    // 解构嵌套的结构体和枚举
    let msg = Message::ChangeColor2(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor2(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor2(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
    println!("Color::Rgb = {:?}", Color::Rgb(1, 2, 3));

    // 解构结构体和元组
    // 如下是一个复杂结构体的例子，其中结构体和元组嵌套在元组中，并将所有的原始类型解构出来
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet = {}, inches = {}, x = {}, y = {}", feet, inches, x, y);

    // 使用嵌套的 _ 忽略部分值
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }

    // 通过在名字前以一个下划线开头来忽略未使用的变量
    let _x = 5; // 不会警告
    let s = Some(String::from("Hello!"));
    // 以下划线开头的未使用变量仍然会绑定值，内部 s 会获取外部 s 值的所有权
    if let Some(_s) = s {
        println!("found a string");
    }
    // 下面这段会编译报错
    // println!("{:?}", s);

    // 用 .. 忽略剩余值
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("Some numbers: {}, {}", first, last),
    }

    // 匹配守卫提供的额外条件
    // 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。匹配守卫用于表达比单独的模式所能允许的更为复杂的情况
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("x is {}", x),
        None => (),
    }

    // 使用匹配守卫来解决模式中变量覆盖的问题，那里 match 表达式的模式中新建了一个变量而不是使用 match 之外的同名变量。新变量意味着不能够测试外部变量的值
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // 匹配守卫 if n == y 并不是一个模式所以没有引入新变量。这个 y 正是 外部的 y 而不是新的覆盖变量 y
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    // 在匹配守卫中使用 或 运算符 | 来指定多个模式，同时匹配守卫的条件会作用于所有的模式
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ 绑定
    // at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
    let hello = Hello::Hi { id: 5 };
    match hello {
        Hello::Hi {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Hello::Hi { id: 10..=12 } => println!("Found an id in another range"),
        Hello::Hi { id } => println!("Found some other id: {}", id),
    }
}
