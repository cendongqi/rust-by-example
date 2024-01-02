fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string an `{:?}`", i);
        },
        _ => {}, // 必需的，看上去有点浪费空间
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // if-let可处理仅一种match的情况，而不需要像match模式那样必须全部处理
    if let Some(i) = number { // if-let只match其中一个并执行，否则不执行
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i)
    } else { // else代表处理匹配失败的分支
        println!("Didn't match a number.");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters { // 解构错误的时候会进入else if分支，比如i不是数字
        println!("Didn't match a number.");
    } else { // 这里是i能解构但是匹配的是None
        println!("I don't like letters.")
    }
}