fn main() {
    // 占位符：{}
    println!("{} days", 30);

    // 位置占位符：{number}
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 名称占位符：{name}，后面的参数和顺序无关
    println!("{subject} {verb} {object}", object="lazy dog", subject="the quick brown fix", verb="jumps over");

    // 通过:指定格式化类型
    println!("Base 10: {}", 69420);
    println!("Base 2: {:b}", 69420);
    println!("Base 8: {:o}", 69420);
    println!("Base 16: {:x}", 69420);
    println!("Base 16: {:X}", 69420);

    // 向右对齐，使用空格填充
    println!("{number:>5}", number=1);

    // 指定字符串填充
    println!("{number:0>5}", number=1);
    println!("{number:s>5}", number=1);

    // 向左对齐，使用0填充
    println!("{number:0<5}", number=1);

    // 通过指定参数指定宽度，并且要用$符号
    println!("{number:0<width$}", number=1, width=10);

    // 实现fmt::Display的自定义类型才可以使用占位符{}，自定义类型默认不实现
    #[allow(dead_code)] // 指定dead_code来忽略编译器警告未被使用的代码
    struct Structure(i32);

    // 代码不会编译，因为未实现fmt::Display
    // println!("This struct `{}` won't print...", Structure(3));

    // 1.58及以上可以读取变量
    let number = 1.0;
    let width: usize = 5;
    println!("{number:0<width$}");

    // 指定小数位数，点.语法 + 小数位长度
    let pi: f64 = 3.141592;
    println!("{pi:.decimals$}", decimals=3);
}