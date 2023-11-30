use std::mem;

// 借用数组的slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 固定长度的数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 使用相同的值初始化数组
    let ys: [i32; 500] = [0; 500];

    // 数组是栈分配
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动借用为slices
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // slice可以指向数组的某部分, 语法为：[start..end]
    println!("Borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // 空slice
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // 更详细但等价的写法

    // 使用`.get`安全的访问数组元素，返回option
    // 或使用`.expect`在发生错误时退出程序
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            None => println!("Slow down! {} is too far!", i),
            Some(xval) => println!("{}: {}", i, xval),
        }
    }

    // 索引越界在数组时编译错误
    // println!("{}", xs[6]);
    // 索引越界在slice时运行时错误
    // println!("{}", &xs[1..2][5])
}