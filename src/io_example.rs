// 输入输出
#[cfg(test)]
mod tests {

    use std::env;
    use std::error::Error;
    use std::fs;
    use std::process;

    struct Config {
        query: String,
        filename: String,
        case_sensitive: bool,
    }

    impl Config {
        // 错误信息的生命周期：所有的字符串字面量都拥有 'static 生命周期
        fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            // main 中的 args 变量是参数值的所有者并只允许 new 函数借用他们，这意味着如果 Config 尝试获取 args 中值的所有权将违反 Rust 的借用规则
            // 而最简单但有些不太高效的方式是调用这些值的 clone 方法。这会生成 Config 实例可以拥有的数据的完整拷贝，不过会比储存字符串数据的引用消耗更多的时间和内存
            // 不过拷贝数据使得代码显得更加直白因为无需管理引用的生命周期，所以在这种情况下牺牲一小部分性能来换取简洁性的取舍是值得的
            let query = args[1].clone();
            let filename = args[2].clone();

            // 读取环境变量，用 Result 的 is_err 方法来检查其是否是一个 error
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config {
                query,
                filename,
                case_sensitive,
            })
        }

        // 使用迭代器的方式获取 args 参数
        fn new_instance(mut args: std::env::Args) -> Result<Config, &'static str> {
            // 将 new 函数改为获取一个有所有权的迭代器作为参数而不是借用 slice
            // 一旦 Config::new 获取了迭代器的所有权并不再使用借用的索引操作，就可以将迭代器中的 String 值移动到 Config 中，而不是调用 clone 分配新的空间
            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };

            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file name"),
            };

            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config {
                query,
                filename,
                case_sensitive,
            })
        }
    }

    // 告诉 Rust 函数 search 返回的数据将与 search 函数中的参数 contents 的数据存在的一样久。
    // 这是非常重要的！为了使这个引用有效那么 被 slice 引用的数据也需要保持有效；
    // 如果编译器认为我们是在创建 query 而不是 contents 的字符串 slice，那么安全检查将是不正确的
    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        results
    }

    // 使用迭代器适配器的方式编写代码，函数式编程风格
    fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
    }

    // 使用迭代器适配器的方式编写代码，函数式编程风格
    fn search_case_insensitive_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query))
            .collect()
    }

    // trait 对象 Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型
    // 这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。这也就是 dyn，它是 “动态的”（“dynamic”）的缩写
    // Ok(()) 表示成功则返回空元组，表明无需关注该函数的返回值，只需要处理其带来的副作用即可
    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        // 不同于遇到错误就 panic!，? 会从函数中返回错误值并让调用者来处理它
        let contents = fs::read_to_string(config.filename)?;

        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results {
            println!("line = {}", line);
        }

        Ok(())
    }

    // 使用迭代器适配器的方式编写代码，函数式编程风格
    fn run_iter(config: Config) -> Result<(), Box<dyn Error>> {
        // 不同于遇到错误就 panic!，? 会从函数中返回错误值并让调用者来处理它
        let contents = fs::read_to_string(config.filename)?;

        let results = if config.case_sensitive {
            search_iter(&config.query, &contents)
        } else {
            search_case_insensitive_iter(&config.query, &contents)
        };

        for line in results {
            println!("line = {}", line);
        }

        Ok(())
    }

    #[test]
    fn io_test() {
        // args 函数返回一个传递给程序的命令行参数的 迭代器（iterator）
        // 在迭代器上调用 collect 方法将其转换为一个集合，collect 可以被用来创建很多类型的集合，所以这里显式注明 args 的类型来指定我们需要一个字符串 vector
        let args: Vec<String> = env::args().collect();
        println!("args = {:?}", args);

        // unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理：
        // 1. 当 Result 是 Ok 时，这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值
        // 2. 当其值是 Err 时，该方法会调用一个 闭包（closure），也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数
        let config = Config::new(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            // 非零中断
            process::exit(1);
        });

        // 因为 run 在成功时返回 ()，而我们只关心检测错误，所以并不需要 unwrap_or_else 来返回未封装的值，因为它只会是 ()
        if let Err(e) = run(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }

        // 迭代器版本
        let config = Config::new_instance(env::args()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        if let Err(e) = run_iter(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
