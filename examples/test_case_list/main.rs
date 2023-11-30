

// write!对结构体的每一个内部数据进行顺序处理，处理起来比较棘手，因为它的返回值是Result，需要处理所有的Result才能保证格式正确
// 提供了?符号来解决，中间出现错误直接返回error，而不会继续执行

use std::fmt;
use std::fmt::{Display, Formatter};

struct List(Vec<i32>);
struct List2(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // 创建vec
        let vec = &self.0; // 这里要用&，否则会发生move

        // 拼接[
        write!(f, "[")?;

        // 使用tuple解构vec，第一个为索引，第二个为值
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ",")?; }
            write!(f, "{}", v)?;
        }
        // 拼接]
        write!(f, "]")
    }
}

impl Display for List2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // 创建vec
        let vec = &self.0; // 这里要用&，否则会发生move

        // 拼接[
        write!(f, "[")?;

        // 使用tuple解构vec，第一个为索引，第二个为值
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }
        // 拼接]
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    let v2 = List2(vec![1, 2, 3]);
    println!("{}", v);
    println!("{}", v2);
}