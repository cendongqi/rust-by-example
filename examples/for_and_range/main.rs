// 创建一个迭代器最简单的方法是使用 a..b语法，包含a，不包含b

use std::mem::needs_drop;

fn main() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 可以包含end的语法，使用：a..=b，既包含a也包含b
    // for i in 1..=101

    // for in 有几种方式可以与Iterator交互
    // 默认情况下，for循环使用into_iter()操作集合
    // 其余两种方式：iter()和iter_mut()

    // iter()
    // 每次迭代都是借用，因此循环结束后集合仍旧可用

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us"), // name是一个引用，所以要匹配引用
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // into_iter()
    // 每次迭代消耗都会转移，集合在迭代后不可用

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names); // 编译错误


    // into_mut()
    // 每次迭代都是可变借用，可以直接修改集合的元素
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name { // name是可变引用，修改它指向的数据先用*解引用后修改
                             // 对于复杂类型，可变引用可以直接修改字段的值；对于基本数据类型，需要解引用后才能操作指向的值
            &mut "Ferris" => "This is a rustacean among us",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}