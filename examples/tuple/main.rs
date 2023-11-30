// tuple是一个不同类型的值的集合
// 函数可以使用tuple来表示多个返回值

use std::fmt::{Display, Formatter};

// tuple可以当参数也可以当返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // let也可以绑定tuple
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {}, {} )\n( {}, {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    // 定义一个长tuple
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 通过索引取值并打印
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // tuple可以嵌套
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // tuple可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 长度超过12的tuple不可以打印, 编译错误
    // let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    // println!("too long tuple: {:?}", too_long_tuple); compiler error
    // You can only format tuples holding up to 12 elements.

    // 测试reverse
    let pair = (5, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    // 创建只有一个元素的tuple，需要保留逗号，否则就不是tuple
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // tuple的解构可以绑定多个变量
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix:");
    println!("{}", matrix);
    println!("Transpose:");
    println!("{}", transpose(matrix));
}