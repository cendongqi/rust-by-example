// closure可以作为入参，也可以作为出参，但是closure的类型是未知的，需要使用impl Trait的方式return
// move关键字也一定要使用，标记所有的捕获都是捕获值。因为所有的引用在function结束后都被dropped了

fn create_fn() -> impl Fn() { // 使用impl Fn()标记返回closure
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() { // 使用impl FnMut()标记返回closure
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() { // 使用impl FnOnce()标记返回closure
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}