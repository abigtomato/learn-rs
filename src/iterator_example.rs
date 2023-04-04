// 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait。这个 trait 的定义看起来像这样
pub trait Iterator {
    type Item;

    // next 是 Iterator 实现者被要求定义的唯一方法。next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None
    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了方法的默认实现
}

// 迭代器
#[cfg(test)]
mod tests {

    // 迭代器性能示例：解码算法使用线性预测数学运算（linear prediction mathematical operation）来根据之前样本的线性函数预测将来的值
    #[test]
    fn decode_example() {
        // 数据切片
        let mut arr = [1, 3, 5, 7, 9];
        let buffer: &mut [i32] = &mut arr[0..3];
        // 12 个元素的数组
        let coefficients: [i64; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        // 位移位数
        let qlp_shift: i16 = 12;

        for i in 12..buffer.len() {
            // 代码遍历了 coefficients 中的 12 个值，使用 zip 方法将系数与 buffer 的前 12 个值组合在一起
            // 接着将每一对值相乘，再将所有结果相加，然后将总和右移 qlp_shift 位
            // 遍历 coefficients 的值完全用不到循环：Rust 知道这里会迭代 12 次，所以它“展开”（unroll）了循环。展开是一种移除循环控制代码的开销并替换为每个迭代中的重复代码的优化
            // 所有的系数都被储存在了寄存器中，这意味着访问他们非常快。这里也没有运行时数组访问边界检查。所有这些 Rust 能够提供的优化使得结果代码极为高效
            let prediction = coefficients
                .iter()
                .zip(&buffer[i - 12..i])
                .map(|(&c, &s)| c * s as i64)
                .sum::<i64>()
                >> qlp_shift;
            let delta = buffer[i];
            buffer[i] = prediction as i32 + delta;
        }
    }

    #[test]
    fn iterator_example() {
        let v1 = vec![1, 2, 3];
        // 迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    #[test]
    fn iterator_demonstration() {
        // 迭代器的next方法：
        // 1. v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态
        // 2. 换句话说，代码 消费（consume）了，或使用了迭代器。每一个 next 调用都会从迭代器中消费一个项
        // 3. 但使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变
        // 4. iter 方法生成一个不可变引用的迭代器。如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。类似的，如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        // 消费迭代器的方法：
        // 1. 这些调用 next 方法的方法被称为 消费适配器（consuming adaptors），因为调用他们会消耗迭代器。一个消费适配器的例子是 sum 方法。
        // 2. 这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和
        // 3. 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_adaptors() {
        // 产生其他迭代器的方法：
        // 1. Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），他们允许我们将当前迭代器变为不同类型的迭代器。
        // 2. 可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果
        // 3. map 方法使用闭包来调用每个元素以生成新的迭代器，但迭代器适配器是惰性的，需要消费迭代器。比如使用 collect 方法消费迭代器并将结果收集到一个数据结构中
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // 调用了 into_iter 来创建一个获取 vector 所有权的迭代器
        // 接着调用 filter 将这个迭代器适配成一个只含有那些闭包返回 true 的元素的新迭代器
        // 闭包从环境中捕获了 shoe_size 变量并使用其值与每一只鞋的大小作比较，只保留指定大小的鞋子
        // 最终，调用 collect 将迭代器适配器返回的值收集进一个 vector 并返回。
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    #[test]
    fn filters_by_size() {
        // 使用闭包获取环境示例
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    // 实现 Iterator trait 来创建自定义迭代器
    impl Iterator for Counter {
        // 将迭代器的关联类型 Item 设置为 u32，意味着迭代器会返回 u32 值集合
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn calling_next_directly() {
        // 使用 Counter 迭代器的 next 方法
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        // 通过定义 next 方法实现 Iterator trait，我们现在就可以使用任何标准库定义的拥有默认实现的 Iterator trait 方法了，因为他们都使用了 next 方法的功能
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
