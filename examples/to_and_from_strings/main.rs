// 任何类型转换成string只需要简单的实现ToString trait即可
// 更好的方式是实现fmt::Display，自动提供了ToString的实现，还允许{}占位符打印

use std::fmt;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::str::FromStr;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// 字符串转自定义类型，必须实现FromStr trait才可以。标准库的某些类型已经内置了这种转换方法
impl FromStr for Circle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<i32, _> = s.parse(); // 这里显示的ParseIntError是由上下文推断出来的，因为下面使用了"10".parse::<Circle>().unwrap()
        match result {
            Ok(i) => Ok(Circle { radius: i }),
            Err(_) => Err(()),
        }
    }
}


// 字符串转数字
// 一般使用parse函数来解析字符串，类型推断或`turbofish`语法声明来指定转换的目标类型
// turbofish语法：::<T>

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // 字符串转数字
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();


    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let circle_from_str = "10".parse::<Circle>().unwrap();
    println!("{}", circle_from_str);
}