// 错误
#[cfg(test)]
mod tests {

    use std::fs::{self, File};
    use std::io::{self, ErrorKind, Read};

    // 传播（propagating）错误：当编写一个需要先调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理
    fn read_username_from_file_1() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // ? 运算符和 match 表达式的不同点：
    // 1. ? 运算符所使用的错误值被传递给了 from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。
    // 2. 当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型。
    // 3. 这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败。只要每一个错误类型都实现了 from 函数来定义如何将自身转换为返回的错误类型，? 运算符会自动处理这些转换
    fn read_username_from_file_2() -> Result<String, io::Error> {
        // ? 被定义为与上面 match + Result 有着完全相同的工作方式。如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut s = String::new();
        // 在 ? 之后直接使用链式方法调用来进一步缩短代码
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    fn read_username_from_file_4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    #[test]
    fn error_example() {
        // 当执行 panic! 宏时，程序会打印出一个错误信息，展开并清理栈数据，然后接着退出
        // 1. 当出现 panic 时，程序默认会开始 展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作
        // 2. 另一种选择是直接 终止（abort），这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理
        // panic!("error occured");

        // 使用 panic! 的回溯（backtrace）
        // 1. 可以设置 RUST_BACKTRACE 环境变量（设置成任何不是0的值）来得到一个回溯
        // 2. 阅读 backtrace 的关键是从头开始读直到发现你编写的文件。这就是问题的发源地。这一行往上是你的代码所调用的代码；往下则是调用你的代码的代码。这些行可能包含核心 Rust 代码，标准库代码或用到的 crate 代码
        // 3. RUST_BACKTRACE=1 cargo run

        // 可恢复错误，可通过Result<T, E>枚举类作返回值来进行异常表达
        let path = "hello.txt";
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(path) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };
        println!("文件句柄 file: {:?}", file);

        // 和上面有着一样的行为，但更易阅读的写法，使用 unwrap_or_else 消除大量嵌套的 match
        let file = File::open(path).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(path).unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
        println!("文件句柄 file: {:?}", file);

        // 失败时 panic 的简写：unwrap 和 expect
        // 1. unwrap，它的实现就类似于上面的 match 语句。如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
        let f1 = File::open(path).unwrap();
        // 2. expect 与 unwrap 的使用方式一样：返回文件句柄或调用 panic! 宏。expect 在调用 panic! 时使用的错误信息将是我们传递给 expect 的参数，而不像 unwrap 那样使用默认的 panic! 信息
        let f2 = File::open(path).expect("failed to open");
        println!("f1 = {:?}, f2 = {:?}", f1, f2);

        // 调用方处理 Result
        match read_username_from_file_1() {
            Ok(s) => println!("read_username_from_file_1 success = {}", s),
            Err(e) => println!("read_username_from_file_1 failed = {}", e),
        }
        match read_username_from_file_2() {
            Ok(s) => println!("read_username_from_file_2 success = {}", s),
            Err(e) => println!("read_username_from_file_2 failed = {}", e),
        }
        match read_username_from_file_3() {
            Ok(s) => println!("read_username_from_file_3 success = {}", s),
            Err(e) => println!("read_username_from_file_3 failed = {}", e),
        }
        match read_username_from_file_4() {
            Ok(s) => println!("read_username_from_file_4 success = {}", s),
            Err(e) => println!("read_username_from_file_4 failed = {}", e),
        }
    }
}
