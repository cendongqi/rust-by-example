// structs有三种类型：tuple struct, c struct, unit struct(无field)

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit struct
struct Unit;

// tuple struct
struct Pair(i32, f32);

// 两个field的struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// struct可以嵌套
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Point{x : tlx, y: tly} = rectangle.top_left;
    let Point{x : brx, y: bry} = rectangle.bottom_right;
    let length = brx - tlx;
    let width = tly - bry;
    length.abs() * width.abs()
}

fn square(point: Point, no: f32) -> Rectangle {
    let bottom_right = Point{ x: point.x + no, y: point.y + no };
    Rectangle { top_left: point, bottom_right }
}

fn main() {
    // 简洁模式创建实例，变量与field同名直接用名字
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    // 实例point
    let point: Point = Point { x: 10.3, y: 0.4 };

    // 访问实例的field
    println!("point coordinates: ({}, {})",  point.x, point.y);

    // 使用已有实例创建新实例
    let bottom_right = Point { x: 5.2, ..point };

    // bottom_right除了x其他字段都可以point一样
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 解构, 把struct对应的field绑定到变量
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: left_edge },
        bottom_right,
    };

    // 实例化unit struct
    let _unit = Unit;

    // 实例化tuple struct，用()
    let pair = Pair(1, 0.1);

    // 访问tuple struct的元素，使用索引
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构tuple struct，无需指定field，直接用变量名，和struct的解构有点区别
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rectangle area is {:.3}", rect_area(_rectangle));
    println!("square is {:?}", square(point, 20.0));
}