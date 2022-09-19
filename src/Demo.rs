// use std::str::FromStr;
//
// /**
// 复数：实数+虚数
// 实数：有理数+无理数
// 有理数：整数+有限小数+无限循环小数
// 无理数：无限不循环小数
// 整数：0+正数+负数
// 虚数： 对负数开平方根得到的数是虚数，也就是说反过来虚数的平方还是负数。
// * 第一个多线程程度
//
// */
// ///
// ///
// ///
// fn main() {
//     println!("基本类型");
//     let big_val = std::i32::MAX;
//     // let x=big_val+1;   //这样写会诧异
//     let x = big_val.wrapping_add(1);
//     println!("溢出之后的翻转值{:?}", x);
//
//     struct Person { name: String, birth: u32 }
//     ;
//     let mut composers = Vec::new();
//     composers.push(Person { name: "plaseti".to_string(), birth: 1512 });
//     composers.push(Person { name: "dowland".to_string(), birth: 888 });
//     composers.push(Person { name: "lully".to_string(), birth: 20 });
//
//     for item in &composers {
//         println!("{},born {}", item.name, item.birth);
//     }
// }
//
// ///
// /// rust的赋值会把变量所有者从圆所有者转移给目标所有者,源所有者置位 未初始化状态
// ///
// fn transfer() {
//     let s = vec!["青青园中葵".to_string(), "百川东到海".to_string(), "少壮不努力".to_string()];
//
//     let t = s;// 执行完这行代码，上面向量的所有者变成了 t，同时 s被置位 未初始化状态
//
//     // 这行代码会报错，因为这时候s 是一个未初始化的值，不能编译通过
//     // let u=s;
//
//     // 如果要完全拥有s 的副本并且不影响s 当前的所有权
//     let u = t.clone();
//
//     let mut a = "小轩窗";
//     a = "正梳妆";//  这行代码执行之后， 上面个字符串会被清除
//
//     // 下面这种情况就不会有值被清除
//     let mut b = "相顾无言";
//
//     let c = b;
//
//     let d = "惟有泪千行";
// }
//
// ///
// /// 在复杂控制流中转移的基本原则
// /// 1.
// ///
// fn transfer2() {
//     let x = vec!["十年生死两茫茫".to_string(), "不思量".to_string()];
//     let a = 5;
//     if a > 0 {
//         let a = x;   // 这里没问题
//     } else {
//         let b = x;  // 这里没问题
//     }
//     //let c=x;  // 这行编译会报错，因为变量x 经过if..else分支后被转移了，是未初始化状态
//
//     let mut x = vec!["自难忘".to_string(), "千里孤坟".to_string(), "何处话凄凉".to_string()];
//     let a = 5;
//     while a > 0 {
//         // let b=x;  //  循环里面这样用不行，因为经过第一次循环之后 变量x 被转移了，是未初始化状态
//     }
//
//     while a > 0 {
//         let b = x;
//         x = vec!["纵使相逢应不识".to_string()]; //这样可以，每次循环 变量x 又被初始化了新值
//     }
// }
//
// ///
// /// 向量中某些下标元素所有权的转移
// ///
// fn transfer3() {
//     let mut v = Vec::new();
//     for i in 101..106 {
//         v.push(i.to_string());
//     }
//
//     // let third = v[2];
//     // let fifth = v[4];
//     // 上面代码执行完之后，向量下标 3，5的元素已经变成未初始化的了. 实际上上面的代码不能编译通过
//
//     // 如果只是想查看值，不想转移，可以用引用
//     let third = &v[2];// 可以编译通过
//
//     // 如果真想转移值，可以用以下方式
//
//     // 1. 取向量末尾元素
//     let c = v.pop().unwrap();
//     println!("c={}", c);
//
//     // 2.取某个下标的值，用最后一个元素替代原来下标的位置
//     let d = v.swap_remove(1);
//     println!("d={}", d);
//
//     // 3. 用指定的值 交换 被转移出来的下标位置
//     let e = std::mem::replace(&mut v[2], "料得年年肠断处".to_string());
//     println!("e={}", e);
//
//     for mut f in v {
//         println!("最后向量剩下元素={}", f);
//     }
// }
//
// #[test]
// fn test_tranf3() {
//     transfer3();
// }
//
// fn transfer4() {
//     struct Person{name:Option<String>, birth:i32};
//
//     let mut composers = Vec::new();
//     composers.push(Person { name: Some("煮豆持作羹".to_string()), birth: 22 });
//
//   //  let first_name=composers[0].name;// 这样会报错，因为结构体Name属性类型已经是option
//
//     let first_name = std::mem::replace(&mut composers[0].name, None);
//     println!("first_name={:?}", first_name);
//
//     println!("{:?}", composers[0].name);
//
//     // 上面的replace方法可以简写为
//
//     let mut composes2 = Vec::new();
//     composes2.push(Person { name: Some("漉豉以为汁".to_string()), birth: 23 });
//
//     let firstname2 = composes2[0].name.take();
//
//     println!("firstname2={:?}", firstname2);
//
//     println!("{:?}", composes2[0].name);
//
// }
//
//
// #[test]
// fn test_tranf4() {
//     transfer4();
// }
//
//
