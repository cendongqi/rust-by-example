

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    // 不同于any()，find()会把迭代器元素的引用传递给闭包，原本迭代器的元素已经是&T，所以再引用就是&&T
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x== 2));
    // into_iter()中迭代器的元素是T，由find变成了&T
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的操作类似
    // iter()解出&&i32
    // into_iter()解出&i32
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x== 2));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x== 2));
}