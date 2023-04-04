// 结构体
#[cfg(test)]
mod tests {

    // 经典的 C 语言风格结构体（structure，缩写成 struct）
    #[derive(Debug)]
    struct User {
        // 字段（field）
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // 函数参数和结构体字段同名可省略字段名
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // 元组结构体（tuple struct），事实上就是具名元组而已
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    // 单元结构体（unit struct），不带字段，在泛型中很有用
    #[derive(Debug)]
    struct AlwaysEqual;

    // 结构体数据的所有权：
    // 1. User的字段使用了自身拥有所有权的 String 类型而不是 &str 字符串 slice 类型，因为想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的
    // 2. 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上生命周期（lifetime）
    // 3. 生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的
    #[test]
    fn struct_test() {
        // 实例化结构体
        let mut user = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        println!("struct instance: user = {:#?}", user);

        // 可变结构体实例可使用点号赋值
        user.active = false;
        user.username = String::from("anotheremail");
        user.email = String::from("anotheremail@example.com");
        user.sign_in_count = 2;
        println!("struct mutable field: user = {:#?}", user);

        // 使用 let 绑定来解构结构体
        let User {
            active,
            username,
            email,
            sign_in_count,
        } = user;
        println!(
            "active = {}, username = {}, email = {}, sign_in_count = {}",
            active, username, email, sign_in_count
        );

        // 字段初始化简写语法（field init shorthand）
        let user = build_user(
            String::from("someone@example.com"),
            String::from("someusername123"),
        );
        println!("field init shorthand: user = {:#?}", user);

        // 结构体更新语法（struct update syntax）
        // 1. 在创建 new_user 后不能再使用 user，因为 user 的 username 字段中的 String 被移到 new_user 中
        // 2. 如果给 new_user 的 email 和 username 都赋予新的 String 值，从而只使用 user 的 active 和 sign_in_count 值，那么 user 在创建 new_user 后仍然有效
        // 3. 因为 active 和 sign_in_count 的类型是实现 Copy trait 的类型
        let new_user = User {
            email: String::from("another@example.com"),
            // 使用现有实例的同名值为新实例赋值
            ..user
        };
        println!("struct update syntax = {:#?}", new_user);

        // 元组结构体（tuple struct）
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        println!("black = {:?}, origin = {:?}", black, origin);

        // 类单元结构体（unit-like structs）没有任何字段的结构体
        // 常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
        let subject = AlwaysEqual;
        println!("subject = {:?}", subject);
    }
}
