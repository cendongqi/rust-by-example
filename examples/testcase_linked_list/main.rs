use crate::List::*;

enum List {
    // tuple struct: 包含一个元素和指向下一个节点的指针
    Cons(u32, Box<List>),
    // 代表linked list的结尾
    Nil,
}

// 枚举可以有method
impl List {
    // 创建一个空list
    fn new() -> List {
        Nil
    }

    // 拿走一个元素，并把原来的list作为新的list返回
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 计算list的长度

    // &self和*self的区别
    // &self: 是一个引用，而不是具体的数据
    // *self: 获取引用的数据以进行模式匹配，但不会发生move
    // match需要匹配的数据而不是引用，所以要用*self
    //

    // &和ref在match中的区别
    // &在match中用于匹配并解构引用
    // ref在match用于匹配并创建引用
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    // 以String列表的形式返回一个list的元素
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}