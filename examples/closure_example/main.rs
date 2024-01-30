// Iterator::any是一个接收iterator的function，如果任何元素满足predicate，返回true，否则返回false

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // iter()迭代的每个元素都是引用&i32，解引用为i32
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // into_iter()迭代的每个元素都会move，不需要解引用
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // iter()仅仅对vec1及元素进行借用，所以可以多次使用
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    // into_iter()对vec2的元素进行了move，vec2不再可用
    println!("First element of vec2 is: {}", vec2[0]);
    println!("vec2 len: {}", vec2.len());
}