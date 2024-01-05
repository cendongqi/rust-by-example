use std::str::FromStr;
use std::u64;

// let-else的作用和if let else类型，更简洁的写法

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'")
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: {count_str}")
    };
    (count, item)
}

fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}