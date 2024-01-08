// 闭包的规则：
// 1. 使用||围绕变量，而不是()
// 2. 对于单行表达式来说，{}是可选的，非单行必有
// 3. 捕获外部环境变量的能力

fn main() {
    let outer_var = 42;

    // 常规函数不可以捕获到closing environment的变量
    // closing environment指的是函数/闭包被定义的作用域中的其他变量或状态
    // ide提示可以用closure代替
    // fn function(i: i32) -> i32 { i + outer_var }

    // closures是匿名的，可以绑定到引用
    // 类型注解和函数类型注解相同是可选的，就像body的`{}`一样
    // 这些匿名函数赋值给适当的变量
    let closure_annotated = |i: i32| -> i32 { i + outer_var}; // 注解
    let closure_inferred = |i| i + outer_var ; // 推断

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {:?}", closure_inferred(1));

    // closure的类型自动推断后，不可以再annotation
    // println!("closure_inferred: {}", closure_inferred(42i64));

    // 没有参数的closure的返回类型推断
    let one = || 1;
    println!("closure returning one: {}", one());
}