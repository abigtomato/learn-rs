// 枚举
#[cfg(test)]
mod tests {

    // 在函数定义中使用泛型
    fn get_zero<T>(list: &[T]) -> &T {
        &list[0]
    }

    // 在结构体定义中使用泛型
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // 多泛型结构体定义，可以在定义中使用任意多的泛型类型参数
    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    // 在枚举定义中使用泛型
    // Option<T> 是一个拥有泛型 T 的枚举，它有两个成员：Some，它存放了一个类型 T 的值，和不存在任何值的 None
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        // None,
    }

    // 多泛型类型枚举
    // 这个定义使得 Result 枚举能很方便的表达任何可能成功（返回 T 类型的值）也可能失败（返回 E 类型的值）的操作
    // #[derive(Debug)]
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // 在方法的定义中使用泛型
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // 这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }

    // 当意识到代码中定义了多个结构体或枚举，它们不一样的地方只是其中的值的类型的时候，不妨通过泛型类型来避免重复
    #[test]
    fn generics_example() {
        let number_list = vec![34, 50, 25, 100, 65];
        println!("get_zero number = {}", get_zero(&number_list));

        let char_list = vec!['y', 'm', 'a', 'q'];
        println!("get_zero char = {}", get_zero(&char_list));

        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        println!("integer.x = {}, integer.y = {}, float.x = {}, float.y = {}, integer.x() = {}, distance_from_origin = {}",
            integer.x, integer.y, float.x, float.y, integer.x(), float.distance_from_origin());

        let integer_and_float = Point2 { x: 5, y: 4.0 };
        println!(
            "integer_and_float.x = {}, integer_and_float.y = {}",
            integer_and_float.x, integer_and_float.y
        );

        let p1 = Point2 { x: 5, y: 10.4 };
        let p2 = Point2 { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

        // 泛型代码的性能：
        // 1. Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程
        // 2. 编译器寻找所有泛型代码被调用的位置并使用泛型代码针对具体类型生成代码
        // 3. 当 Rust 编译这些代码的时候，它会进行单态化。编译器会读取传递给 Option<T> 的值并发现有两种 Option<T>：一个对应 i32 另一个对应 f64。为此，它会将泛型定义 Option<T> 展开为 Option_i32 和 Option_f64，接着将泛型定义替换为这两个具体的定义
        // 4. 我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。这意味着在使用泛型时没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样
        let integer = Option::Some(5);
        let float = Option::Some(5.0);
        println!("integer = {:?}, float = {:?}", integer, float);
    }
}
