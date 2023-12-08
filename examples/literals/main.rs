// 数字字面量类型标记的另一种形式——后缀标记，比如：42i32
// 没有类型标记的数字字面量取决于它如何使用，默认情况下整数是i32，浮点数是f64

use std::mem::size_of_val;

fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    // size_of_val返回变量占用的字节数
    // 参数用引用代表更通用，可以兼容非基本类型
    // 这些变量都是基本数据类型，实现了copy trait，传递的都是拷贝后的值
    println!("size of `x` in bytes: {}", size_of_val(&x));
    println!("size of `y` in bytes: {}", size_of_val(&y));
    println!("size of `z` in bytes: {}", size_of_val(&z));
    println!("size of `i` in bytes: {}", size_of_val(&i));
    println!("size of `f` in bytes: {}", size_of_val(&f));
}