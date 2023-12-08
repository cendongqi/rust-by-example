use crate::Work::Civilian;

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char), // 类似tuple struct
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(web_event: WebEvent) {
    match web_event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed \"{}\".", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

// 类型别名
#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;


// use
// 使用use可以避免手动作用域限定
#[allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// c-like enums
// 默认情况下，discriminator默认给每个variant赋予一个整数值，从0开始
// 隐式discriminator的枚举（索引或标识符）
enum Number {
    Zero,
    One,
    Tow,
}

// discriminator也可以显式地指定整数值
// 显式discriminator的枚举
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 30, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    println!("{:?}", x);

    // 显式使用use而不需要全限定名
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor; // 等效 Status::Poor
    let work = Civilian; // 等效Work：：Civilian

    match status {
        Rich => println!("the rich have lots of money"),
        Poor => println!("the poor have no money"),
    }

    match work {
        Civilian => println!("Civilians work"),
        Soldier => println!("Soldiers fight"),
    }

    // 枚举可以转换成整数
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}