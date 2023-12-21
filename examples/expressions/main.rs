// 一个程序由多个statement组成

fn main() {
    // statement
    // statement
    // statement

    // 最常见的两种expression，一个中变量绑定，一种是使用;
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    // block也是expression，最后一个表达式如果不带;就返回值，和变量绑定一样；如果带;就返回()
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 这个表达式把值赋给y
        x_cube + x_squared + x
    };

    let z = {
        // 这个表达式讲把()赋值给z
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}