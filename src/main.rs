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
}
