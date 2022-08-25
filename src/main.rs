use std::str::FromStr;

/**
复数：实数+虚数
实数：有理数+无理数
有理数：整数+有限小数+无限循环小数
无理数：无限不循环小数
整数：0+正数+负数
虚数： 对负数开平方根得到的数是虚数，也就是说反过来虚数的平方还是负数。
* 第一个多线程程度

*/
///
///
///
fn main() {
    println!("基本类型");
    let big_val = std::i32::MAX;
    // let x=big_val+1;   //这样写会诧异
    let x = big_val.wrapping_add(1);
    println!("溢出之后的翻转值{:?}", x);

    // 类型转换
    let v : f64=3.2;

    println!("f64值{:?} 类型转换为 {:?}", v, v as f32);
    println!("-1.01f64.floor={}", -1.01f64.ceil());
    println!("-1.01f64.floor={}", -1.01f64.floor());
    println!("布尔转换为整数={}", false as i32);
    println!("布尔转换为整数={}", true as u32);
}

///
/// Rust可以根据类型推断函数大部分的值，实践中，对于某个变量或表达式通常只有一种类型的值适用。此时，
/// Rust允许我们忽略其他变量的类型声明。如下
///
fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(16i16);
    v.push(20i16);
    v
}

///
/// 简写版本，和上面的函数完全等价且编译的机器码也一样
///
fn build_vector_easy() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(16);
    v.push(22);
    v
}


///
///
///
fn print_slice(n: &[f64]) {
    for item in n {
        println!("{}", item);
    }
}

#[test]
fn test_print_slice() {
    let n0: Vec<i32> = (0..9).collect();
    let n: Vec<f64> = vec![6.2, 3., 5.6, 7.9];
    let sn: &[f64] = &n;
    print_slice(sn);

    let arr: [f64;5] = [2., 4., 8., 9., 10.];
    let sarr: &[f64] = &arr;
    print_slice(sarr);

    let mut v_capacity: Vec<u32> = Vec::with_capacity(5);
    v_capacity.push(2);
    v_capacity.push(5);
    v_capacity.push(6);
    v_capacity.push(8);
    v_capacity.push(8);
    v_capacity.push(8);

    println!("可容纳数量={}", v_capacity.capacity());
    println!("实际数量={}", v_capacity.len());
}
