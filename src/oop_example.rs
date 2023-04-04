// 面向对象
#[cfg(test)]
mod tests {

    // 面向对象的程序是由对象组成的。一个 对象 包含数据和操作这些数据的过程。这些过程通常被称为 方法 或 操作
    // 在这个定义下，Rust 是面向对象的：结构体和枚举包含数据而 impl 块提供了在结构体和枚举之上的方法。虽然带有方法的结构体和枚举并不被 称为 对象，但是他们提供了与对象相同的功能
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn new(list: Vec<i32>, average: f64) -> AveragedCollection {
            AveragedCollection {
                list: list,
                average: average,
            }
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            match self.list.pop() {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }

    // 一个通常与面向对象编程相关的方面是 封装（encapsulation）的思想：
    // 1. 对象的实现细节不能被使用对象的代码获取到。
    // 2. 所以唯一与对象交互的方式是通过对象提供的公有 API；使用对象的代码无法深入到对象内部并直接改变数据或者行为。
    // 3. 封装使得改变和重构对象的内部时无需改变使用对象的代码
    #[test]
    fn encapsulation() {
        let mut ac = AveragedCollection::new(vec![], 0.0);
        ac.add(233);
        ac.add(666);
        ac.remove();
        ac.average();
    }

    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // 这个 vector 的类型是 Box<dyn Draw>，此为一个 trait 对象：它是 Box 中任何实现了 Draw trait 的类型的替身
        // 这与定义使用了带有 trait bound 的泛型类型参数的结构体不同。泛型类型参数一次只能替代一个具体类型，而 trait 对象则允许在运行时替代多种具体类型
        // 如果只需要同质（相同类型）集合，则倾向于使用泛型和 trait bound，因为其定义会在编译时采用具体类型进行单态化
        // 通过使用 trait 对象的方法，一个 Screen 实例可以存放一个既能包含 Box<Button>，也能包含 Box<TextField> 的 Vec<T>
        // 只有 对象安全（object safe）的 trait 才可以组成 trait 对象，如果一个 trait 中所有的方法有如下属性时，则该 trait 是对象安全的：
        // 1. 返回值类型不为 Self
        // 2. 方法没有任何泛型类型参数
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    #[warn(dead_code)]
    struct Button {
        width: u32,
        height: u32,
        label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!(
                "width = {}, height = {}, label = {}",
                self.width, self.height, self.label
            );
        }
    }

    #[warn(dead_code)]
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!(
                "width = {}, height = {}, options = {:?}",
                self.width, self.height, self.options
            );
        }
    }

    // 继承（Inheritance）是一个很多编程语言都提供的机制，一个对象可以定义为继承另一个对象的定义，这使其可以获得父对象的数据和行为，而无需重新定义
    // 选择继承有两个主要的原因。第一个是为了重用代码：一旦为一个类型实现了特定行为，继承可以对一个不同的类型重用这个实现。相反 Rust 代码可以使用默认 trait 方法实现来进行共享
    // 第二个使用继承的原因与类型系统有关：表现为子类型可以用于父类型被使用的地方。这也被称为 多态（polymorphism），这意味着如果多种对象共享特定的属性，则可以相互替代使用
    #[test]
    fn inheritance() {
        // 创建一个 Screen 实例。至此可以通过将 SelectBox 和 Button 放入 Box<T> 转变为 trait 对象来增加组件
        // 接着可以调用 Screen 的 run 方法，它会调用每个组件的 draw 方法
        // 静态分发发生于编译器在编译时就知晓调用了什么方法的时候。这与 动态分发（dynamic dispatch）相对，这时编译器在编译时无法知晓调用了什么方法。在动态分发的情况下，编译器会生成在运行时确定调用了什么方法的代码
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };
        screen.run();
    }
}
