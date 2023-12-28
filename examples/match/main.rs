use std::pin::pin;

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
    // 使用 .. 来忽略其中的元素
    let tuple = (0, -2, 3);
    println!("tell me abount {:?}", tuple);
    match tuple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z), // 只解构第二、三个元素
        (1, ..) => println!("First is `1` and the rest doesn't matter"), // 只匹配第一个值
        (.., 2) => println!("Last is `2` and the rest doesn't matter"), // 只匹配最后一个值
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"), // 只匹配第一个和最后一个值
        _ => println!("It doesn't matter what they are"),
    }

    // 解构arrays/slices
    let array = [1, -2, 6];

    match array {
        [0, second, third] => // second和third分别绑定到对应位置的元素
            println!("arrays[0] = 0, arrays[1] = {}, arrays2 = {}", second, third),
        [1, _, third] => // 单个值忽略使用 _
            println!("arrays[0] = 0, arrays2 = {} and array[1] was ignored", third),
        [-1, second, ..] => // 多值忽略使用 ..
            println!("arrays[0] = -1, array[1] = {} and all the other ones were ignored", second),
        // [-1, second] => ... // 数量不一致的匹配编译错误
        [3, second, tail @ ..] => // 使用 @ .. 来把其余位置的元素挪到一个新的arrays/slices中
            println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
        [first, middle @ .., last] => // 组合这些模式
            println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
    }

    // 解构enums
    let color = Color::RGB(122, 17, 40);

    println!("what color is it?");

    match color {
        Color::Red => println!("the color is red"),
        Color::Blue => println!("the color is blue"),
        Color::Green => println!("the color is green"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
    }

    // 解构指针/引用
    let reference = &4;

    match reference { // 匹配引用使用&
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference { // 匹配引用的值使用*解引用
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let value = 5;
    let mut mut_value = 6;
    // 在match的arm中创建引用而不是匹配引用使用ref
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10; // 修改引用的数据需要*
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    // 解构struct
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i), // 变量绑定和顺序和无关，和name油管
        Foo { y, ..} => println!("y = {}, we don't care about x", y), // 忽略其余的field
        // Foo {y} => println!("y = {}", y), // 编译错误，必须要声明其余字段的处理，要么使用..
    }

    let faa = Foo { x: (1, 2), y: 3};
    let Foo{ x: x0, y: y0 } = faa; // 解构struct不一定要在match中，这里把x和y解构到x0和y0中
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}