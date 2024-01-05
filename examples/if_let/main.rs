fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string an `{:?}`", i);
        },
        _ => {}, // 必需的，看上去有点浪费空间
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // if-let可处理仅一种match的情况，而不需要像match模式那样必须全部处理
    if let Some(i) = number { // if-let只match其中一个并执行，否则不执行
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i)
    } else { // else代表处理匹配失败的分支
        println!("Didn't match a number.");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number.");
    } else { // 这里是i能解构但是匹配的是None
        println!("I don't like letters.")
    }

    // if-let 作用于enum
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a { // if-let的匹配可以不需要PartialEq，而是以模式的形式进行匹配，不需要比较实例
        println!("a is foobar");
    }

    if let Foo::Bar = b { // b不会匹配Foo::Bar，所以不会打印
        println!("b is foobar");
    }

    // c会匹配，且c的值会绑定到value
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }


}