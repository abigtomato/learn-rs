// 闭包
#[cfg(test)]
mod tests {

    use std::thread;
    use std::time::Duration;

    // Fn 系列 trait 由标准库提供。所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个
    // 下面的例子中闭包有一个 u32 的参数并返回一个 u32，这样所指定的 trait bound 就是 Fn(u32) -> u32
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    // 带缓存的闭包调用
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        // 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数
        // 不同于函数，闭包允许捕获调用者作用域中的值
        // 闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数
        // 参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号可以省略
        // let 语句意味着 expensive_result 包含一个匿名函数的 定义，而不是调用匿名函数的 返回值
        // 闭包不要求像 fn 函数那样在参数和返回值上注明类型
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }

    #[test]
    fn closures_test() {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);

        // 若不指定具体类型，闭包定义会为每个参数和返回值推断一个具体类型
        let example_closure = |x| x;
        println!(
            "example_closure = {:#?}",
            example_closure(String::from("hello"))
        );
        // 下面这段无法通过编译，因为尝试调用闭包两次，第一次使用 String 类型作为参数而第二次使用 u32
        // let n = example_closure(5);

        // 闭包会捕获其环境：即便 x 并不是 equal_to_x 的一个参数，equal_to_x 闭包也被允许使用变量 x，因为它与 equal_to_x 定义于相同的作用域
        // 闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 Fn trait：
        // 1. FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
        // 2. FnMut 获取可变的借用值所以可以改变其环境
        // 3. Fn 从其环境获取不可变的借用值
        // 当创建一个闭包时，Rust 根据其如何使用环境中变量来推断我们希望如何引用环境。由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce 。那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut ，而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));

        // x 被移动进了闭包，因为闭包使用 move 关键字定义。接着闭包获取了 x 的所有权
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // 下面这段编译无法通过，因为x已经被移动到闭包中
        // println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }
}
