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
fn apply<F>(f: F) where F: Fn() {
    f();
}

/*
    这一个函数接收一个closure并返回i32
 */
fn apply_to_3<F>(f: F) -> i32
    // 这个closure接收一个i32并返回一个i32
    where F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
}