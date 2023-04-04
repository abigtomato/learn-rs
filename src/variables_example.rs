// 变量
#[cfg(test)]
mod tests {
    
    use std::fmt::{self, Display};
    use std::mem;

    // 全局变量是在所有其他作用域之外声明的
    static LANGUAGE: &'static str = "Rust";
    const THRESHOLD: i32 = 10;

    // 变量的可变与不可变
    #[test]
    fn immutable_and_mutable() {
        // 不可变变量（immutable）
        // 变量都能够显式地给出类型说明（type annotation）
        let a: i64 = 123;
        println!("immutable a = {:b}", a);

        // 可变变量（mutable）
        // 数字还可以通过后缀（suffix）或默认方式（整型默认为 i32 类型，浮点型默认为 f64类型）来声明类型
        let mut b = 3.14f32;
        println!("mutable b = {0}", b);
        b = 3.15;
        println!("mutable b = {0}", b);

        // Rust 有两种常量（constant），可以在任意作用域声明，包括全局作用域。它们都需要显式的类型声明
        // 1. const：不可改变的值（通常使用这种）
        // 2. static：具有 'static 生命周期的，可以是可变的变量（译注：须使用 static mut 关键字）
        const C: i32 = 123;
        println!("C = {}", C);
        // 有个特例就是 "string" 字面量。它可以不经改动就被赋给一个 static 变量，因为它 的类型标记：&'static str 就包含了所要求的生命周期 'static
        println!("LANGUAGE = {}, THRESHOLD = {}", LANGUAGE, THRESHOLD);

        // 变量的重影/遮蔽（variable shadowing）
        let d: u64 = 123;
        println!("shadow d = {shadow}", shadow = d);
        let d: f64 = 3.15;
        println!("shadow d = {}", d);
    }

    // 标量类型（scalar type）
    // 1. 有符号整数（signed integers）：i8、i16、i32、i64、i128 和 isize（指针宽度）
    // 2. 无符号整数（unsigned integers）： u8、u16、u32、u64、u128 和 usize（指针宽度）
    // 3. 浮点数（floating point）： f32、f64
    // 4. char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
    // 5. bool（布尔型）：只能是 true 或 false
    // 6. 单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组
    #[test]
    fn scalar_test() {
        // 取决于程序运行的计算机体系结构，表示为“arch”：若使用 64 位架构系统则为 64 位，若使用 32 位架构系统则为 32 位。
        let a: isize = 98_222;
        // 十六进制
        let b: u64 = 0xff;
        // 有符号数字以二进制补码形式存储
        let c: i64 = 0o77;
        // 有符号类型-(2^n-1^) ~ 2^n-1^ - 1 其中 n 是该定义形式的位长度，所以 i8 可存储数字范围是 -(27) ~ 27 - 1，即 -128 ~ 127
        let d: i32 = 0b1111_0000;
        // 无符号类型可以存储的数字范围是 0 ~ 2n - 1，所以 u8 能够存储的数字为 0 ~ 28 - 1，即 0 ~ 255
        let e: u8 = b'A';
        let f: bool = false;
        // 字符类型大小为 4 个字节，表示的是一个 Unicode 标量值
        let g: char = '😻';
        println!(
            "a: isize = {}, b: u64 = {}, c: i64 = {}, d: i32 = {}, e: u8 = {}, f: bool = {}, g: char = {}",
            a, b, c, d, e, f, g
        );
    }

    // 此函数借用一个 slice
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    // 复合类型（compound type）
    // 1. 数组（array）：如 [1, 2, 3]
    // 2. 元组（tuple）：如 (1, true)
    #[test]
    fn compound_test() {
        // 元组（tuple）
        // 1. 元组是将多种类型的多个值组合到一个复合类型中的一种基本方式
        // 2. 元组的长度是固定的：声明后，它们就无法增长或缩小
        // 3. 元组使用括号 () 来构造（construct），而每个元组自身又是一个类型标记为 (T1, T2, ...) 的值
        let tuple: (i32, f64, u8) = (500, 6.4, 1);
        println!(
            "tuple = {:?}, tuple.0 = {}, tuple.1 = {}, tuple.2 = {}",
            tuple, tuple.0, tuple.1, tuple.2
        );

        // 元组拆包/解构（destructure）
        let (h, i, j) = tuple;
        println!("tuple destructure: h = {}, i = {}, j = {}", h, i, j);

        // 元组也可以充当元组的元素
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
        println!("tuple_of_tuples = {:?}", tuple_of_tuples);

        // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分
        println!(
            "one element tuple: {:?}, just an integer: {:?}",
            (5u32,),
            (5u32)
        );

        // 该类型被称为单元类型（unit type），该值被称为单元值（unit value）。如果表达式不返回任何其他值，就隐式地返回单元值
        let unit = ();
        println!("unit = {:?}", unit);

        // 数组（array）
        // 1. 数组（array）是一组拥有相同类型 T 的对象的集合，在内存中是连续存储的
        // 2. 数组使用中括号 [] 来创建，且它们的大小在编译时会被确定，数组的类型标记为 [T; length]
        // 3. 当希望将数据分配到栈（stack）而不是堆（heap）时，或者当希望确保始终具有固定数量的元素时，数组特别有用
        let array: [i32; 5] = [1, 2, 3, 4, 5];
        println!(
            "array = {:?}, array[0] = {}, array[1] = {}",
            array, array[0], array[1]
        );

        // 通过指定范围初始化数组
        let arr = [3; 5];
        println!("arr = {:?}", arr);

        // 数组是在栈中分配的
        println!("array occupies {} bytes", mem::size_of_val(&arr));

        // 切片（slice）
        // 1. 切片的大小在编译时是不确定的，是一个双字对象（two-word object），第一个字是一个指向数据的指针，第二个字是切片的长度
        // 2. 这个 “字” 的宽度和 usize 相同，由处理器架构决定，比如在 x86-64 平台上就是 64 位
        // 3. slice 可以用来借用数组的一部分。slice 的类型标记为 &[T]
        analyze_slice(&arr); // 数组可以自动被借用成为 slice
        analyze_slice(&arr[0..2]); // slice 可以指向数组的一部分
    }

    // 字面量和运算符
    #[test]
    pub fn literal_and_operator() {
        // 整数相加减
        println!("1 + 2 = {}", 1u32 + 2);
        println!("1 - 2 = {}", 1i32 - 2);

        // 短路求值的布尔逻辑
        println!("true AND false is {}", true && false);
        println!("true OR false is {}", true || false);
        println!("NOT true is {}", !true);

        // 位运算
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

        // 使用下划线改善数字的可读性！
        println!("One million is written as {}", 1_000_000u32);
    }

    struct List(Vec<i32>);

    // 自定义显示结构
    impl Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (i, v) in vec.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{index}: {value}", index = i, value = v)?;
            }

            write!(f, "]")
        }
    }

    #[test]
    pub fn display_example() {
        let v = List(vec![1, 2, 3]);
        println!("List Vector Display: {}", v);
    }
}
