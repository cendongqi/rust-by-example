/*
    closure必须是泛型的
    编译器隐式创建一个匿名结构体来存储捕获的变量，同时给这个未知的类型实现Fn，FnMut，FnOnce的其中一个，然后将这个类型赋值给某个变量直到被调用
    由于这个类型是未知的，所以它使用的都是泛型。但是一个没有边界的泛型是模糊的，是不被允许的，因此用Fn，FnMut，FnOnce来限制
 */

fn apply<F>(f: F) where F: Fn() {
    f();
}

fn main() {
    let x = 7;
    // 捕获x到一个匿名类型中并实现Fn，存储在print
    let print = || println!("{}", x);

    apply(print);
}