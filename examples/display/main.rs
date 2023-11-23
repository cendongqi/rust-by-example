use std::fmt::{Display, Formatter}; // 导入display

// 定义一个Struct来实现Display
struct Structure(i32);

// 实现Display
impl Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 严格写入第一个元素到输出流：f 中，返回fmt::Result 表示成功或失败
        // write!的用法和println!类似
        write!(f, "{}", self.0)
    }
}

// 因为泛型容器的打印样式不是固定的，std库也不应该预设一个样式，所以std库并不会为如：vec<T>的泛型容器实现Display，
// 而是使用fmt::Debug提供更通用的显示。这样，开发者可以根据需要自定义不同类型的显示方式：如
// Vec<path>: /:/etc:/home/username:/bin (split on :)
// Vec<number>: 1,2,3 (split on ,)

// 对于任意的非泛型容器，fmt::Display仍旧可以实现


// 对比Debug和Display
#[derive(Debug)]
struct MinMax(i64, i64); // 元组结构体

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D { // 字段有名称的普通结构体
    x: f64,
    y: f64,
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let min_max = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", min_max);
    println!("Debug: {:?}", min_max);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and small range is {small}", big=big_range, small=small_range);

    let point = Point2D { x: 5.2, y: 3.6};

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}