fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    // match可以传入匿名变量，并且匹配的是一个数字范围而不是绑定到某个变量商
    // 使用 @ 符号来把值绑定到变量
    println!("tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n=> println!("I'm an old person of age {:?}", n),
    }

    // 解构enum时，也可以绑定到枚举里面的值
    match some_number() {
        Some(n @ 42) => println!("The answer: {}!", n),
        Some(n) => println!("Not interesting...{}", n),
        _ => {}
    }
}