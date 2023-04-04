// 集合
#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Vector允许在一个单独的数据结构中储存多个相同类型的值，所有值在内存中彼此相邻排列
    #[test]
    fn vector_example() {
        // 这里我们增加了一个类型标注。因为没有向这个 vector 中插入任何值，Rust 并不知道我们想要储存什么类型的元素
        let v: Vec<i32> = Vec::new();
        println!("v = {:?}", v);
        // 常见的做法是使用初始值来创建一个 Vec，而且为了方便 Rust 提供了 vec! 宏。这个宏会根据我们提供的值来创建一个新的 Vec
        let v = vec![1, 2, 3];
        println!("v = {:?}", v);

        // 如果想要能够改变它的值，必须使用 mut 关键字使其可变。
        // 放入其中的所有值都是 i32 类型的，而且 Rust 也根据数据做出如此判断，所以不需要 Vec<i32> 标注
        let mut v = Vec::new();
        v.push(666);

        // 丢弃 vector 时也会丢弃其所有元素
        {
            let v = vec![1, 2, 3, 4];
            // 处理变量 v
            println!("v = {:?}", v);
        } // <- 这里 v 离开作用域并被丢弃

        // 读取 vector 的元素
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2]; // 索引语法，返回引用
        println!("The third element is {}", third);
        match v.get(2) {
            // get方法，返回Option<&T>
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }

        // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中
        // 这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        // v.push(6); // 编译错误
        println!("The first element is: {}", first);
        v.push(6); // 编译通过，first最后一次使用之后已经失效

        // 遍历 vector 中的元素
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }

        // 使用枚举来储存多种类型
        // 1.Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存
        // 2.第二个好处是可以准确的知道这个 vector 中允许什么类型
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        println!("row = {:?}", row);
    }

    // Rust 的核心语言中只有一种字符串类型：str，字符串 slice，它通常以被借用的形式出现，&str
    #[test]
    fn string_example() {
        // 空字符串
        let s = String::new();
        println!("s = {}", s);
        // 借用转所有权持有
        // to_string 方法：它能用于任何实现了 Display trait 的类型，字符串字面量也实现了它
        let data = "initial contents";
        let s = data.to_string();
        println!("s = {}", s);
        // 使用 String::from 函数从字符串字面量创建 String
        println!("hello = {}", String::from("السلام عليكم"));
        println!("hello = {}", String::from("Dobrý den"));
        println!("hello = {}", String::from("Hello"));
        println!("hello = {}", String::from("שָׁלוֹם"));
        println!("hello = {}", String::from("नमस्ते"));
        println!("hello = {}", String::from("こんにちは"));
        println!("hello = {}", String::from("안녕하세요"));
        println!("hello = {}", String::from("你好"));
        println!("hello = {}", String::from("Olá"));
        println!("hello = {}", String::from("Здравствуйте"));
        println!("hello = {}", String::from("Hola"));

        // 更新字符串
        let mut s = String::from("foo");
        s.push_str("bar");
        s.push('l');

        // 使用 + 运算符或 format! 宏拼接字符串
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // 1. + 运算符使用了 add 函数，这个函数签名: fn add(self, s: &str) -> String {}
        // 2. &s2 的类型是 &String 而不是 &str，之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str
        // 3. add 函数被调用时，Rust 使用了一个被称为 解引用强制转换（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]
        // 4. add 的签名中 获取了 self 的所有权，因为 self 没有 使用 &。意味着 s1 的所有权会被移动到 add 调用中，之后就不再有效
        let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
        println!("s3 = {}", s3);
        // format! 与 println! 的工作原理相同，不过不同于将输出打印到屏幕上，它返回一个带有结果内容的 String
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s4 = format!("{}-{}-{}", s1, s2, s3);
        println!("s4 = {}", s4);

        // String 是一个 Vec<u8> 的封装
        // UTF-8 编码的 “Здравствуйте” 所需要的字节数是24，这是因为每个 Unicode 标量值需要 2 个字节存储
        // 因此一个字符串字节值的索引并不总是对应一个有效的 Unicode 标量值，所以 Rust 不允许字符串索引
        // 另一个 Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间 (O(1))。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符
        let len = String::from("Здравствуйте").len();
        // let answer = &"Здравствуйте"[0]; // 编译报错
        println!("len = {}", len);

        // 字符串 slice
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        // let s = &hello[0..1]; // 运行时报错
        println!("s = {}", s);

        // Rust以三种方式理解字符串，如梵文书写的印度语单词 “नमस्ते”：
        // 1. 最终储存在 vector 中的 u8 值：[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
        // 2. 从 Unicode 标量值的角度理解：['न', 'म', 'स', '्', 'त', 'े']
        // 3. 以字形簇的角度理解：["न", "म", "स्", "ते"]

        // 遍历字符串的方法
        for c in "नमस्ते".chars() {
            // 此方式遍历获取的是字符串的Unicode 标量值
            println!("c = {}", c);
        }
        for b in "नमस्ते".bytes() {
            // 此方式遍历获取的是字符串最终存储在计算机中的字节
            // 有效的 Unicode 标量值可能会由不止一个字节组成
            println!("b = {}", b);
        }
        // 从字符串中获取字形簇是很复杂的，所以标准库并没有提供这个功能
    }

    // HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射，通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中
    #[test]
    fn map_test() {
        // 新建 HashMap
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("scores = {:?}", scores);

        // 通过 vector 新建 HashMap
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("scores = {:?}", scores);

        // HashMap 和所有权
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        // 1. 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map
        // 2. 对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
        // 3. 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // 这里 field_name 和 field_value 不再有效

        // 访问哈希 map 中的值
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        // 根据 key 获取 value
        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        if let Some(score) = score {
            println!("score = {:?}", score);
        }
        // 遍历 map 中所有的键值对
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        // 更新哈希 map
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        // 覆盖一个值
        scores.insert(String::from("Blue"), 25);
        // 只在键没有对应值时插入：
        // 1. 经常会检查某个特定的键是否有值，如果没有就插入一个值
        // 2. 为此哈希 map 有一个特有的 API，叫做 entry，它获取我们想要检查的键作为参数
        // 3. entry 函数的返回值是一个枚举，Entry，它代表了可能存在也可能不存在的值
        // 4. Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);
        // 根据旧值更新一个值，如WordCount
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            // 1. or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）
            // 2. 这里我们将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count
            // 3. 这个可变引用在 for 循环的结尾离开作用域
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}
