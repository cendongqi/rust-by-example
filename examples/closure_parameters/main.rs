fn apply<F>(f: F) where
    F: FnOnce() { // 声明了泛型F是一个closure，且类型是FnOnce
    f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 { // 声明了F是一个接收i32并返回i32的closure，且函数返回一个i32
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // &str是非copy类型
    // 使用`to_owned`来拷贝一个借用指向的值
    // String::from("")和"".to_owned()功能上相同，语义不同
    // String::from("")的语义：从某个已知的源创建String
    // "".to_owned()的语义：从一个已知的借用创建一个拥有其数据的String，这个方法必须要实现ToOwned trait，可以用于任何实现了这个trait的类型
    let mut farewell = greeting.to_owned();

    let diary = || {
        println!("I said {}.", greeting); // 变量捕获1：Fn

        farewell.push_str("!!!"); // 变量捕获2：FnMut
        println!("Then I screamed {}.", farewell);
    };
}