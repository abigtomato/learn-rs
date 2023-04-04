// 所有权
#[cfg(test)]
mod tests {

    /**
     * 变量的所有权机制
     *
     * 内存管理策略
     * 1. 一些语言中具有垃圾回收机制，在程序运行时不断地寻找不再使用的内存（Java、Golang）
     * 2. 在另一些语言中，开发者必须亲自分配和释放内存（C、C++）
     * 3. Rust 则选择了第三种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。在运行时，所有权系统的任何功能都不会减慢程序。
     *
     * Rust 区别与其他高级语言的重要特征，在于其内存管理的两个特点
     * 1. 变量超出作用域会自动释放：对于简单值类型的栈内存（如int，struct）超出作用域后自动释放，这个逻辑在各个语言都有实现。
     *    而对于 new 出来的堆内存，在c/c++中是要手动释放的，在java中要委托垃圾回收器释放。
     *    但GC不是实时的，且会影响性能（STW），手动释放难以管理。
     *    Rust对堆栈内存一视同仁，超出作用域一律自动释放，这个特点在兼顾性能的情况下有效的减少了代码量和内存泄漏隐患。
     * 2. 变量的所有权机制：某段内存只能被最后一个变量所有，前面声明过的变量都作废，这有效的避免同一块内存被多个变量释放的问题
     *    而且该操作是在编译期就可以检查到的，在编译期就能有效的避免空指针问题。
     *    所有权的设定本质上就是在语言层面禁止了同一个可变数据会有多个变量引用的情况，一旦作为参数传递了，就会发生所有权的移动（Move）或借用（Borrow）。
     *    赋值给另一个变更也就自动放弃了所有权，从根本上杜绝了并发情景下的数据共享冲突。
     *
     * 栈（Stack）与堆（Heap）
     * 1. 在很多语言中，你并不需要经常考虑到栈与堆。不过在像 Rust 这样的系统编程语言中，值是位于栈上还是堆上在更大程度上影响了语言的行为以及为何必须做出这样的抉择
     * 2. 栈以放入值的顺序存储值并以相反顺序取出值。这也被称作 后进先出（last in, first out）。增加数据叫做 进栈（pushing onto the stack），而移出数据叫做 出栈（popping off the stack）
     * 3. 栈中的所有数据都必须占用已知且固定的大小。在编译时大小未知或大小可能变化的数据，要改为存储在堆上。堆是缺乏组织的：当向堆放入数据时，你要请求一定大小的空间。
     *    内存分配器（memory allocator）在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的 指针（pointer）。这个过程称作 在堆上分配内存（allocating on the heap），有时简称为 “分配”（allocating）。
     *    将数据推入栈中并不被认为是分配。因为指针的大小是已知并且固定的，你可以将指针存储在栈上，不过当需要实际数据时，必须访问指针
     * 4. 入栈比在堆上分配内存要快，因为（入栈时）分配器无需为存储新数据去搜索内存空间；其位置总是在栈顶。相比之下，在堆上分配内存则需要更多的工作，这是因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备
     * 5. 访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。现代处理器在内存中跳转越少就越快（缓存）
     * 6. 当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈
     * 7. 跟踪哪部分代码正在使用堆上的哪些数据，最大限度地减少堆上的重复数据量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的
     */
    #[test]
    fn ownership_test() {
        // 所有权规则
        // 1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量
        // 2. 值在任一时刻有且只有一个所有者
        // 3. 当所有者（变量）离开作用域，这个值将被丢弃
        {
            // 在声明以前，变量a无效
            let a = "rust";
            // 从这开始是变量a的有效范围
            println!("a = {}", a);
        }
        // 变量范围已经结束，变量 a 无效
        // Rust之所以没有明示释放的步骤是因为在变量范围结束的时候，Rust编译器自动添加了调用释放资源函数的步骤

        // 基本数据类型在栈空间存储（如像整型这样的在编译时已知大小的类型被整个存储在栈上），x赋值y会直接拷贝
        // Rust 有一个叫做 Copy trait 的特殊标注，可以用在类似整型这样的存储在栈上的类型上。如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用
        // 如下是一些 Copy 的类型：
        // 1. 所有整数类型，比如 u32。
        // 2. 布尔类型，bool，它的值是 true 和 false。
        // 3. 所有浮点数类型，比如 f64。
        // 4. 字符类型，char。
        // 5. 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。
        let x = 5;
        let y = x; // 拷贝（copy）
        println!("x = {}, y = {}", x, y);

        // 长度不确定的数据会在堆中存储，s1赋值s2只移动栈中的指针，同时s1无效（这样做是为了避免二次释放double free的错误，即当s2 和 s1 离开作用域，都会尝试释放相同的内存）
        let s1 = String::from("hello");
        let s2 = s1; // 移动（move）
        println!("s2 = {}", s2);

        // 克隆（clone）/深拷贝（deep copy）
        let s3 = String::from("world");
        let s4 = s3.clone();
        println!("s3 = {}, s4 = {}", s3, s4);
    }

    fn taskes_ownership(some_string: String) {
        // some_string 进入作用域
        println!("{}", some_string);
        // some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
    }

    fn makes_copy(some_integer: i32) {
        // some_integer 进入作用域
        println!("{}", some_integer);
        // 这里，some_integer 移出作用域。不会有特殊操作，随函数出栈
    }

    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        // 返回 some_string 并移出给调用的函数
        some_string
    }

    // takes_and_gives_back 将传入字符串并返回该值
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string 进入作用域
        a_string
        // 返回 a_string 并移出给调用的函数
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        // 返回元组，用返回值交还s的所有权
        (s, length)
    }

    // 变量做为函数参数和返回值时的所有权机制
    #[test]
    fn fun_ownership_test() {
        // 变量做为函数参数时的所有权机制
        let s1 = String::from("test");
        // 从这开始是s1的作用范围
        taskes_ownership(s1);
        // s1被作为参数传入，已被移动，s1无效
        let s2 = 5;
        // 从这开始是s2的作用范围
        makes_copy(s2);
        // s2被作为参数传入，因为是基本类型，会在栈中直接复制一份，因此s2不会失效。当函数结束时，s2才会无效

        // 变量做为函数返回值时的所有权机制
        // gives_ownership的返回值移动给s3
        let s3 = gives_ownership();
        println!("s3 = {}", s3);
        let s4 = String::from("test");
        // 从这开始是s4被声明的有效范围
        // s4被移动到takes_and_gives_back中，函数返回值移动给s5
        let s5 = takes_and_gives_back(s4);
        println!("s5 = {}", s5);

        // 变量的所有权总是遵循相同的模式
        // 1.将值赋给另一个变量时移动它。
        // 2.当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有

        // 返回参数的所有权 函数使用一个值但不获取所有权（传递参数给函数且想在调用后继续使用）
        let s6 = String::from("hello");
        let (s7, len) = calculate_length(s6);
        println!("The length of '{}' is {}.", s7, len);
    }

    fn calculate_length_new(str: &String) -> usize {
        // &String是字符串的引用类型
        str.len()
        // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // 引用和租借
    #[test]
    fn borrowing() {
        let s1 = String::from("hello");
        // &运算符可以取变量的"引用"，允许使用值但不获取其所有权
        let mut s2 = &s1;
        println!("s1 = {}, s2 = {}", s1, s2);
        // &s2创建一个指向s2的引用，可以使用s2指向的值，但不拥有s2，当引用停止使用时，它所指向的值也不会被丢弃
        println!("len = {}", calculate_length_new(&s2));

        // 创建一个引用的行为称为 租借/借用（borrowing），引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，不具有所指值的所有权
        let mut s3 = s1;
        println!("s3 = {}", s3);

        // s1已经将所有权移动到s3，此时s2已经无法继续使用s1的值了，需要重新租借s3
        s2 = &s3;
        println!("s2 = {}", s2);

        // 引用租借的所有权只享有访问的能力，若引用需要修改的能力，则要使用可变引用&mut
        change(&mut s3);
        println!("s3 = {}", s3);

        // 1.同一时间某一特定数据被可变引用时，不能有其他任何引用存在（包含可变和不可变）。不可变引用之间可以有多个，不可变和可变引用互斥
        // 2.Rust对可变引用的这种设计主要出于对并发状态下发生数据竞争（data race）的考虑，在编译阶段就避免了这种情况的发生
        // 3.由于发生数据竞争必要条件之一是数据被至少一个使用者写且同时被至少一个其他使用者读或写，所以在一个值被可变引用时不允许再次被任何引用
        let s4 = &mut s3;
        // let s5 = &s3; // 编译错误，和&mut s3互斥
        // let s6 = &mut s3; // 编译错误，和&mut s3互斥
        println!("s4 = {}", s4);

        // 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
        let s7 = &s3;
        let s8 = &s3;
        println!("s7 = {}, s8 = {}", s7, s8);
        // 此位置之后 s7 和 s8 不再使用
        // s9在此可以被声明，作用域不会发生重叠
        let s9 = &mut s3;
        println!("{}", s9);
    }

    fn dangle() -> String {
        let s = String::from("hello");
        // 这块s的所有权是被移动出去的，内存中的值没有被释放
        s
    }

    // 悬垂指针
    #[test]
    fn dangling_pointer() {
        // 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者
        // Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域
        println!("dangel = {}", dangle());
    }

    // 编译错误，
    // fn dangle() -> &String { // 返回一个字符串类型的指针
    //     let s = String::from("hello");
    //     &s
    // } // s已经离开作用域被丢弃，内存已经被释放，而指向s原本使用的内存区域的指针被返回出去，所以会出现悬垂的情况，即指针指向错误的内存区域

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                // 返回字符串切片
                return &s[0..i];
            }
        }

        &s[..]
    }

    // 切片
    #[test]
    fn slice() {
        // 另一个没有所有权的数据类型是 slice。slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合

        // Rust中的字符串类型实质上记录了字符在内存中的起始位置和其长度
        // x..y 表示 [x, y) 的数学含义
        // ..y 等价于 0..y
        // x.. 等价于位置 x 到结尾
        // .. 等价于位置 0 到结束
        // 被切片引用的字符串禁止更改其值
        let s = String::from("hello world");
        let part1 = &s[0..5];
        let part2 = &s[5..9];
        println!("{} = {} + {}", s, part1, part2);

        // 因为 clear 需要清空 String，它尝试获取一个可变引用。在调用 clear 之后的 println! 使用了 word 中的引用，所以这个不可变的引用在此时必须仍然有效。
        // Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，因此编译失败
        let word = first_word(&s);
        // let word = first_word(&s[0..6]);
        // let word = first_word(&s[..]);
        // s.clear();   // 编译报错
        println!("word = {}", word);

        // &str是字符串切片类型 字符串字面量就是切片
        // 它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面量是不可变的；&str 是一个不可变引用
        let str: &str = "hello";
        // String是字符串类型
        let string: String = String::from("hello");
        println!("str = {}, string = {}", str, string);

        // 切片结果必须是引用类型
        let arr = [1, 3, 5, 7, 9];
        // &[i32]是数组切片类型
        // 跟字符串 slice 的工作方式一样，通过存储第一个集合元素的引用和一个集合总长度
        let part: &[i32] = &arr[0..3];
        for i in part.iter() {
            println!("i = {}", i);
        }
    }
}
