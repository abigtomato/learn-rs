// 宏（Macro）指的是 Rust 中一系列的功能：使用 macro_rules! 的 声明（Declarative）宏，和三种 过程（Procedural）宏：
// 1. 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
// 2. 类属性（Attribute-like）宏定义可用于任意项的自定义属性
// 3. 类函数宏看起来像函数不过作用于作为参数传递的 token
pub fn macro_example() {}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
