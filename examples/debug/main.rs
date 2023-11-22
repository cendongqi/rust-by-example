use std::fmt::DebugStruct;
use std::io::Stderr;

fn main() {
    // fmt::Display仅针对std中的类型有自动实现，自定义类型需要手动实现
    #[allow(dead_code)]
    struct Unprintable(i32);

    // 使用fmt::Debug来打印自定义类型而无需实现fmt::Display
    #[derive(Debug)]
    struct DebugPrintable(i32);

    // 使用{:?}打印debug信息，和{}用法差不多
    #[derive(Debug)]
    struct Structure(i32);
    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?}! name.", "Slater", "Christan", actor="acctor's");

    // 默认输出结构体的值和字段值
    println!("Now {:?} will print.", Structure(3));
    println!("Now {:?} will print.", Deep(Structure(7)));

    // pretty打印： Debug + {:#?}
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person {
        name, age
    };

    println!("{:#?}", peter);
}