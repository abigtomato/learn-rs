// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

/**
 * 类型系统
 * 
 * Rust 提供了多种机制，用于改变或定义原生类型和用户定义类型
 * 1. 原生类型的类型转换（cast）
 * 2. 指定字面量的类型
 * 3. 使用类型推断（type inference）
 * 4. 给类型取别名（alias）
 */
pub fn type_system_example() {
    let decimal = 65.4321_f32;

    // 可以显式转换
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当把任何类型转换为无符号类型 T 时，会不断加上或减去 (std::T::MAX + 1)
    // 直到值位于新类型 T 的范围内。

    // 1000 已经在 u16 的范围内
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 事实上的处理方式是：从最低有效位（LSB，least significant bits）开始保留
    // 8 位，然后剩余位置，直到最高有效位（MSB，most significant bit）都被抛弃。
    // 译注：MSB 就是二进制的最高位，LSB 就是二进制的最低位，按日常书写习惯就是
    // 最左边一位和最右边一位。
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 对正数，这就和取模一样。
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换到有符号类型时，（位操作的）结果就和 “先转换到对应的无符号类型，
    // 如果 MSB 是 1，则该值为负” 是一样的。

    // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 重复之前的例子
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 的二进制补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    // 带后缀的字面量，其类型在初始化时已经知道了。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    // 无后缀的数值字面量，其类型取决于怎样使用它们。如果没有限制，编译器会对整数使用 i32，对浮点数使用 f64
    let i = 1;
    let f = 1.0;
    // `size_of_val` 返回一个变量所占的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

use std::convert::{From, TryFrom, TryInto};
use std::string::ToString;

#[derive(Debug)]
struct Number {
    value: i32,
}

// From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制
// 在标准库中有无数 From 的实现，规定原生类型及其他常见类型的转换功能
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

// 类似于 From 和 Into，TryFrom 和 TryInto 是 类型转换的通用 trait。
// 不同于 From/Into 的是，TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，其返回值是 Result 型
impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

pub fn type_transformation_example() {
    // From trait
    let num = Number::from(30);
    println!("My number is {:?} value = {}", num, num.value);

    // Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into
    // 使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    // TryFrom trait
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    
    // TryInto trait
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // ToString trait
    let circle = Circle { radius: 6 };
    println!("circle = {}", circle.to_string());

    // 我们经常需要把字符串转成数字。完成这项工作的标准手段是用 parse 函数
    // 得提供要转换到的类型，这可以通过不使用类型推断，或者用 “涡轮鱼” 语法（turbo fish，<>）实现
    // 只要对目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型。 标准库中已经给无数种类型实现了 FromStr。如果要转换到用户定义类型，只要手动实现 FromStr 就行
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!{"parsed + turbo_parsed = sum: {:?}", sum};
}

pub fn senior_type_example() {}
