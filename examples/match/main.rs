fn main() {
    let number = 13;

    println!("tell me about {}", number);
    match number {
        1 => println!("One"), // 匹配单个值
        2 | 3 | 5 | 7 | 11 | 13 => println!("this is a prime"), // 匹配多个值
        13..=19 => println!("a teen"), // 匹配一个范围
        _ => println!("Ain't special"), // 剩下的所有情况
    }

    let boolean = true;
    let binary = match boolean { // 一般情况下需要处理所有可能的值
        true => 1,
        false => 0,
    };

    println!("boolean -> {}, binary -> {}", boolean, binary);

    // match的解构
    // 解构tuples, arrays/slices, enums, pointers/ref, structs

    // 解构tuples
    let tuple = (0, -2, 3);
    println!("tell me abount {:?}", tuple);
    match tuple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z), // 只解构第二、三个元素
        (1, ..) => println!("First is `1` and the rest doesn't matter"), // 只匹配第一个值
        (.., 2) => println!("Last is `2` and the rest doesn't matter"), // 只匹配最后一个值
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"), // 只匹配第一个和最后一个值
        _ => println!("It doesn't matter what they are"),
    }
}