fn main() {
    // loop关键字表示一个无线循环
    // 使用break和continue进行跳出和跳过
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count+= 1;
        if count == 3 {
            println!("three");

            continue
        }
        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    // nesting 和 label
    // 嵌套loop循环的break和continue，可以指定跳出哪个循环，循环使用'label来标记
    'outer: loop {
        println!("enter the outer loop");

        'inner: loop {
            println!("enter the inner loop");

            break 'outer;
        }

        println!("this point will never be reached");
    }

    // loop的break可以返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    assert_eq!(result, 20);
}