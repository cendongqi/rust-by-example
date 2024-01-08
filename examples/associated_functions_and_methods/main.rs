// associated functions通常定义在类型里
// methods是一种被类型的instance调用的associated function
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // 这是一个associated function，它关联的是具体的类型，也就是Point
    // associated function不需要instance来调用，就像构造器一样
    fn origin() -> Point {
        Point { x: 0.00, y: 0.00}
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x , y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // method的特征：第一个参数就是self
    // &self其实就是self: &Self的语法糖，它代表的是类型的调用对象，在这里 Self = Rectangle
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2} = self.p2;
        ((x1 - x2) * (y1 - y2).abs())
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2} = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first , second) = self;

        println!("Destroy Pair({}, {})", first, second);
        // first和second离开后被释放，整个Pair也失效了
    }
}

fn main() {
    let rectangle = Rectangle {
        // associated function使用双冒号调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 实例可以调用method
    // 注意method的self是隐式传参
    // rectangle.perimeter() 等同于 rectangle.perimeter(&rectangle)
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    // rectangle.translate(1.0, 0.0); // translate需要的是可变的对象，而rectangle是不可变对象，编译错误

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 0.0); // square是可变的，可以随意修改

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
    // pair.destroy(); // 编译报错，因为第一条destroy之后，pair已经释放了
}