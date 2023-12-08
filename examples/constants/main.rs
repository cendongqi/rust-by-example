// 有两种常量
// const: 不可改变的值
// static: 带有'static生命周期的可变变量，生命周期自动推断无需指定，访问和修改一个可变的static是不安全的

// 全局声明
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在主线程中访问常量
    println!("{}", LANGUAGE);
    println!("{}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 常量不能修改
    // THRESHOLD = 5;
}