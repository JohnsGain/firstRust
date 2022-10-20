use plant_struct::leaves::*;
use plant_struct::leaves::Tree;

//  告诉rust编译器，sports.rs是一个模块
use crate::sports::*;

mod sports;
///
/// 在 main.rs 声明   plant_struct  模块，rust会去加载 plant_struct/mod.rs ，这个mod.rs  又声明了
/// 3个 子模块
///
pub mod plant_struct;

///
///
///
///
fn main() {
    let mut factory = vec!["a".to_string(), "b".to_string()];
    let s = produce_spore(&mut factory);

    // let v = recombine(&mut factory);   //  not accessible b私有的函数，不能访问

    // * ::  操作符 用于访问 模块的特性，项目中任何地方都可以通过写出绝对路径来引用标准库特性:
    let mut s1 = 5;
    let mut s2 = 8;
    if s1 > s2 {
        ::std::mem::swap(&mut s1, &mut s2);
    }

    // 声明一个结构体值
    let mut peach = Tree { name: "peach".to_string(), age: 32 };
    peach.fruit();
    peach.fruit();

    let mut consturct = Tree::new();
    consturct.name = "banana".to_string();
    consturct.age = 32;
    consturct.fruit();

    // 操作符重载
    let tree2 = Tree { name: "peach".to_string(), age: 100 };
    println!("peach==tree2? {}", peach == tree2);
    println!("peach==tree2? {}", peach.eq(&tree2));

    let tree3 = Tree { name: "peach".to_string(), age: 100 };
    println!("tree3==tree2? {}", tree3 == tree2);
    println!("tree3==tree2? {}", tree3.eq(&tree2));

    // 泛型结构体
    let forest = Forest { name: "amazon".to_string(), trees: vec! {"oak", "peach"} };
    let forest2 = Forest { name: "xishuangbanna".to_string(), trees: vec! {tree2, tree3} };

    // 实现默认值 特性
    // let forest3 = Forest::<Tree>::new();  会报错  泛型需要实现Default特型的类型
    let forest3 = Forest::<Vec<i8>>::new();
    println!("forest3={:?}", forest3);
}
