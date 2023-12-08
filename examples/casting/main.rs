// 忽略所有转换引起的溢出警告
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Rust中不存在隐式转换
    // let integer: u8 = decimal;

    // 显式转换，使用as关键字
    let integer = decimal as u8;
    let charater = integer as char;

    // 转换也有规则，浮点数不能转成char
    // let character = decimal as char;

    println!("casting -> {} -> {}", decimal, integer/*, character*/);

    // 转换任意一个值为无符号类型T时，若改值的范围超过了这个类型能表示的范围，那么这个值会和T::max + 1相加（负数时）或相减（正数时）直到这个值落到这个类型的范围区间
    // 1000在u16范围内
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232，i32 -> u8转换行为的等效
    // 实际上的计算机内部实现：最低的8位保留，其余位截断或丢弃，结果就是232
    println!("1000 as a u8 is: {}", (1000i32) as u8);

    // -1 + 256 = 255
    // 这里-1先用有符号整数表示
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // 对于正数，as和取模的效果一样
    println!("1000 mod 256 is: {}", 1000 % 256);

    // 值转成有符号类型时，结果和转换成无符号类型相同，如果最高位是1，那就是负数
    println!("128 as a i16 is: {}", 128 as i16);

    // 在边界的情况下，128在8位的二进制补码中会显示为-128，i8(-128, 127);
    println!("128 as a i8 is: {}", (128i16) as i8);

    // 从1.45版本开始，使用as将浮点数转换成整数时，会执行饱和式转换（saturating cast），超过上边界时直接返回上边界值，超过下边界返回下边界值
    println!("300.0 as u8 is: {}", 300.0_f32 as u8); // 255
    println!("-100.0 as u8 is: {}", -100.0_f32 as u8); // 0
    // 空返回下限边界值
    println!("nan as u8 is: {}", f32::NAN as u8); // 0

    // 这种行为（浮点数转整数超过范围返回边界值）会带来一些运行时开销，通过unsafe方法可以避免这种开销，但是可能会导致值溢出，并返回不可靠的值，谨慎使用
    unsafe {
        println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>()); // 44
        // 不使用括号的优先级是100.0_f32，才到-号，加上括号才
        println!("-100.0 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>()); // 156
        println!("Nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>()); // 0
    }
}