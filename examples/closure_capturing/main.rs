// closure捕获变量的三种方式：
// &T
// &mut T
// T
// 首选引用，需要的时候才会降级

use std::mem;

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

    // let _reborrow = &count; // 编译错误，后面的代码又执行了可变引用，这里不能借用不可变引用
    inc();

    let _count_reborrowed = &mut count; // 后面没有再执行count，可以重新借用了


    let movable = Box::new(3);
    /*
        mem::drop需要的是T而不是&T，需要所有权转移，编译器推断这里需要做所有权转移
        而原始的数字3属于拷贝类型，这样的T被closure捕获时，会发生Copy，不会转移，对副本进行mem::drop没有意义
        所以这里要创建智能指针Box::new来防止拷贝行为
     */
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable); // mem::drop的语义是拿到所有权之后丢弃
    };

    consume(); // 只能调用一次，因为movable释放了
}