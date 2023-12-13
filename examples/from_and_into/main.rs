// From trait定义了如何一个类型如何从其他类型创建它自己
// 如果从A能转到B，那么应该从B也能转到A

// from用在自定义类型
#[allow(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// Into trait和From trait是互补的关系，在实现了From的同时会自动实现Into，此时显式实现Into会发生编译错误
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

fn main() {
    // str转String
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("my_string is {}", my_string);

    // 自定义类型
    let num = Number::from(30);
    println!("My num is {:?}", num);

    // i32转Number
    let int = 5;
    let num: Number = int.into(); // 没有足够的上下文推断出类型，这里要显式指定类型
    println!("My number is {:?}", num);
}