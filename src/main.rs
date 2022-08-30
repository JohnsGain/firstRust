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

    struct Person{name:String, birth: u32};
    let mut composers = Vec::new();
    composers.push(Person { name: "plaseti".to_string(), birth: 1512 });
    composers.push(Person { name: "dowland".to_string(), birth: 888 });
    composers.push(Person { name: "lully".to_string(), birth: 20 });

    for item in &composers {
        println!("{},born {}", item.name, item.birth);
    }

}
///
/// rust的赋值会把变量所有者从圆所有者转移给目标所有者,源所有者置位 未初始化状态
///
fn transfer() {
    let s = vec!["青青园中葵".to_string(), "百川东到海".to_string(), "少壮不努力".to_string()];

    let t= s;// 执行完这行代码，上面向量的所有者变成了 t，同时 s被置位 未初始化状态

    // 这行代码会报错，因为这时候s 是一个未初始化的值，不能编译通过
    // let u=s;

    // 如果要完全拥有s 的副本并且不影响s 当前的所有权
    let u = t.clone();

    let mut  a ="小轩窗";
    a="正梳妆";//  这行代码执行之后， 上面个字符串会被清除

    // 下面这种情况就不会有值被清除
    let mut  b = "相顾无言";

    let c= b;

    let d="惟有泪千行";


}

