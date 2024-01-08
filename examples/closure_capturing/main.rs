// closure捕获变量的三种方式：
// &T
// &mut T
// T
// 首选引用，需要的时候才会降级

fn main() {
    let color = String::from("green");

    // closure存储color的引用直到最后的print执行
    let print = || println!("color: {}", color);

    print(); // 执行closure

    let _reborrow = &color; // 可以再次借用，因为closure只持有引用，可以创建多个不可变引用
    print();

    let _color_moved = color; // 在最后的print之后，color允许移动或重新借用


    let mut count = 0;
    // count既可以使用&mut count也可以直接使用count，但是&mut count的优先级更高
    // inc内部有mut变量，意味着inc可以改变它内部的状态或捕获的变量，所以inc也需要声明为mut
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc(); // 使用可变引用调用closure

    // let _reborrow = &count; // 编译错误，后面的代码又执行了
    inc();
}