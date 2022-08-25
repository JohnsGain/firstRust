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
    let v: f64 = 3.2;

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
    let v: Vec<f64> = vec![6.2, 3., 5.6, 7.9];
    // 把整个向量的引用给了切片
    let sn: &[f64] = &v;
    print_slice(sn);

    // 只给部分范围的元素引用给切片
    let sn2: &[f64] = &v[2..];
    print_slice(sn2);


    let arr: [f64; 6] = [2., 4.23, 8., 9., 10., 6.88];
    // 把整个数组的引用给了切片
    let sarr: &[f64] = &arr;
    print_slice(sarr);

    // 只给部分范围的元素引用给切片
    let sarr2: &[f64] = &arr[1..6];
    print_slice(sarr2);

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

///
/// 字符串字面量，双引号需要转义,  字符串可以分散到多行..
/// 多行字符串带左斜杠拼接，类似于java中的 + 号，实际字符串是连着一起的，只是为了可读性分在多行展示。
///
/// Rust提供了 原始字符串 语法，原始字符串里面的任何内容都按照原样保存，不做转义。
///
fn string() {
    let mut speech = "everything can be possible";
    println!("{}", speech);
    speech = "i can say:\"everything can be possible!\" ";
    println!("{}", speech);

    speech = "in the room come and go,\
    Singging of mount abora";
    println!("{}", speech);

    speech = "in the room come and go,
    Singging of mount abora";
    println!("raw_str={}", speech);

    // 原始字符串, r###"[这里的内容是原始字符串文本]"###
    let raw_str = r###"i can say:"everything can be possible!\\" Singing of mount ab###ora "###;
    println!("{}", raw_str);
}

#[test]
fn test_string() {
    string();
}

///
/// 字节字符串 ：就是前缀带b 的字符串字面量。字节字符串 是 u8（即字节） 值的切片，
/// 不是unicode文本的切片. 字节字符串不能包含任意unicode字符，只能是 ascii 和\xHH 转移序列
///
/// 字符串字面量的值存在堆内存，字符串本身有3个头信息(指向堆内存里面字面量第一个字符的引用地址，
///     字符串容量，字符串实际字符字节长度)存储在栈内存。
///
///  &str  ，字符串切片，对其他utf8文本某部分字符序列的引用。也是胖指针，
/// 有两个头信息(向堆内存里面字面量第一个字符的引用地址，引用部分实际字符的字节长度)存储在栈内存，
///
/// String &str 的len()方法返回的是字节长度，不是字符个数。 不要试图修改 &str指向引用的部分字符值，编译会报panic
///  &str类似于 数组、向量切面 &[T]，  String类似于 Vec<T>
///
fn byte_str() {
    let method = b"GET";
    println!("methond={:?}", method);
    assert_eq!(method, &[b'G', b'E', b'T']);

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "(￣.￣)";

    println!("字节长度={:?}", noodles.len());
    println!("字节长度={}", oodles.len());
    println!("字符长度={}", poodles.chars().count());
    println!("字节长度={}", poodles.len());

    // 试图修复 字符串切片 下标0 的值，会编译Panic
    let mut s = "hello";
    // s[0] = 'A';
    // s.push('x');

    // 以下方式可以创建字符串
    let hello = "hello".to_string(); // 实际上是复制字符串
    // format!()宏 和 println!类型，区别是他返回一个新String,而不是直接把文本标准输出，并且不会自动在末尾加换行符
    let longitude = format!("{}°{:02}'{:02}\"N", 24, 5, 23);
    println!("{}", longitude);

    // 字符串数组,
    let bits = vec!["veni", "bidi", "john"];
    println!("{}", bits.concat());
    println!("{}", bits.join("-"));

    // 字符串比较。支持 ==， !=,<, >,>=,<=  只要字符串包含相同字符，顺序。那么就相等，无论他们是否指向 同一个内存地址
    assert!("ONE".to_lowercase() == "one");
    println!("{}", "相对无言，欲语泪千行".contains("泪"));
    println!("{}", "相对无言，(￣.￣)".replace("￣", "🌝"));
    println!("{}", "相对无言，(￣.￣)\n");
    println!("{}", "相对无言，(￣.￣)   ".trim());

    let word_comma = "夜，来，幽，梦，忽，还，乡";
    let split = word_comma.split("，");
    for word in split {
        println!("{}", word);
    }

}

#[test]
fn test_byte_string() {
    byte_str();
}
