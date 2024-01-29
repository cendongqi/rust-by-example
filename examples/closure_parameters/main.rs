/*  Rust在大多数情况下不需要type annotation来动态选择如何捕获变量，但是在函数中不可以这么模棱两可
    当closure作为输入参数时，必须有一种特性来注明闭包的完整类型，它们来决定closure捕获的value的类型
    按照限制程度递减：Fn(&T)、FnMut(&mut T)、FnOnce(T)
    编译器以最少限制为基准选择变量，最终决定的类型以closure如何使用变量为准
    如果move是可能发生的，那么借用一定可以发生，反过来则不行
        比如：参数标记为Fn，那么FnMut和FnOne都不允许使用
 */

/*
    F是泛型，where进一步说明F的类型时closure，有三种：Fn()、FnMut()、FnOnce()
 */
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
        println!("Now I can sleep. zzzz");

        // 因为drop需要的参数是value，所以这里的farewell确定了是FnOnce
        drop(farewell); // 变量捕获3：FnOnce
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 double: {}", apply_to_3(double));
}