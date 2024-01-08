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

    let _color_moved = color; // 在最后的print之后，color允许移动或重新借用了
    let mut count = 0;

}