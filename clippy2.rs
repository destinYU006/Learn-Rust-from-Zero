// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in option.iter() {
        res += x;
    }
    println!("{}", res);
}
//option 是单个 Option 类型，不是可迭代对象
// for 循环期望的是实现了 IntoIterator 的类型
// option.iter()返回一个迭代器：如果 option 是 Some(value)，迭代器包含一个元素 &value
//if  option 是 None，迭代器为空;循环体内的 x 类型是 &i32，Rust 会自动解引用执行加法运算
//另一种更惯用的写法是使用 if let：    
//  if let Some(x) = option {
//         res += x;
//     }
