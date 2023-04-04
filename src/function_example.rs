// 函数
#[cfg(test)]
mod tests {

    // 在函数签名中，必须声明每个参数的类型
    fn print_labeled_measurement(value: i32, unit_label: char) -> i32 {
        println!("The measurement is: {}{}", value, unit_label);
        3
    }

    // Rust 代码中的函数和变量名使用下划线命名法（snake case，直译为蛇形命名法）规范风格
    #[test]
    fn function_test() {
        // 表达式块
        let k = {
            let l = 3;
            l + 1
        };
        println!("{}", k);

        // 嵌套函数
        fn five() -> i32 {
            5
        }
        println!("{}", five());

        // 函数调用
        print_labeled_measurement(5, 'c');
    }
}
