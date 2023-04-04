use std::fmt;
use std::ops::Add;

pub trait Iterator {
    // 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型
    // trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型
    // 如此可以定义一个使用多种类型的 trait，直到实现此 trait 时都无需知道这些类型具体是什么
    type Item;

    // 和泛型类型的区别：
    // 1. 当 trait 有泛型参数时，可以多次实现这个 trait，每次需改变泛型参数的具体类型。接着当使用 Counter 的 next 方法时，必须提供类型标注来表明希望使用 Iterator 的哪一个实现
    // 2. 通过关联类型，则无需标注类型，因为不能多次实现这个 trait。能选择一次 Item 会是什么类型，因为只能有一个 impl Iterator for Counter。当调用 Counter 的 next 时不必每次指定我们需要 u32 值的迭代器
    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 运算符重载（Operator overloading）是指在特定情况下自定义运算符（比如 +）行为的操作
// 不过 std::ops 中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载
// 在 Point 结构体上实现 Add trait 来重载 + 运算符，这样就可以将两个 Point 实例相加
impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

// trait Add<RHS=Self>：RHS=Self 这个语法叫做 默认类型参数（default type parameters）
// RHS 是一个泛型类型参数（“right hand side” 的缩写），它用于定义 add 方法中的 rhs 参数。如果实现 Add trait 时不指定 RHS 的具体类型，RHS 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型
// 下面的代码目的是提供 Millimeters 和 Meters 相加的能力
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Rust 既不能避免一个 trait 与另一个 trait 拥有相同名称的方法，也不能阻止为同一类型同时实现这两个 trait
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

// Human实现trait Pilot的fly方法
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

// Human实现trait Wizard的fly方法
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// Human自己的方法
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// 在 outline_print 的实现中，因为希望能够使用 Display trait 的功能，则需要说明 OutlinePrint 只能用于同时也实现了 Display 并提供了 OutlinePrint 需要的功能的类型
// 可以通过在 trait 定义中指定 OutlinePrint: Display 来做到这一点。这类似于为 trait 增加 trait bound
// 因为指定了 OutlinePrint 需要 Display trait，则可以在 outline_print 中使用 to_string， 其会为任何实现 Display 的类型自动实现
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Point 仅仅实现 OutlinePrint trait 是不够的
impl OutlinePrint for Point {}

// 同时要在 Point 上实现 Display trait 才能满足 OutlinePrint 要求的限制
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 高级trait特性
pub fn senior_trait_example() {
    // 运算符重载
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    println!("p1 + p2 = {:?}", p1 + p2);

    let millimeters = Millimeters(12);
    let meters = Meters(11);
    println!("millimeters + meters = {:?}", millimeters + meters);

    // 完全限定语法与消歧义：调用相同名称的方法
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // 父 trait 用于在另一个 trait 中使用某 trait 的功能
    let p3 = Point { x: 4, y: 5 };
    p3.outline_print();
}
