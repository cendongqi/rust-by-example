// Try_from和Try_into类似，不同的是返回值都是Result，用于处理需要错误处理的类型转换

#[derive(Debug, PartialEq)] // 这里需要PartialEq是因为assert_eq!需要PartialEq来比较实例相等性，恰恰这里就是比较实例，Result已经默认实现了PartialEq，泛型也需要实现
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}