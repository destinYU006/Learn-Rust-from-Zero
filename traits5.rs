// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func<T:SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
// //  泛型参数和多重trait约束
// 代码解释：
// 泛型参数与多重 trait 约束：
// 使用泛型参数T: SomeTrait + OtherTrait表示函数可以接受任何同时实现了SomeTrait和OtherTrait的类型。
// +符号用于组合多个 trait 约束。
// 函数调用：
// SomeStruct和OtherStruct都同时实现了这两个 trait，因此可以作为参数传递给some_func。
// 关键点：
// 多重 trait 约束：通过T: Trait1 + Trait2语法，要求类型必须同时满足多个 trait 约束。
// 默认实现：由于两个 trait 都提供了默认实现，所有实现该 trait 的类型都会自动拥有对应的方法。
