// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let mut res = 42;       //res会被分配 4 个字节的空间（这是因为 i32 类型占 4 字节），并且初始值为 42。
    let option = Some(12);  //option则会分配 8 个字节（因为 Option<i32>属于枚举类型，包含一个 i32 值和一个用于判别是否为 Some 的标记位，共占 8 字节），它会被初始化为 Some (12)。
    for x in option.iter() { //迭代器存储：在栈上，option.iter()会生成一个迭代器，不过这个迭代器是零大小的，也就是说它不会占用实际的内存空间。
//循环变量x是对 option 内部值的引用，它在栈上会占用 8 个字节（这是 64 位系统上引用的大小）。
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
// 编译器处理过程
// 类型检查：编译器会确认option的类型是Option<i32>，而option.iter()返回的是一个迭代器，该迭代器会产生&i32类型的值。
// 循环展开：由于option是Some(12)，所以循环体只会执行一次。要是option的值为None，循环体就不会执行。
// 借用检查：编译器会保证在option被借用期间，其值不会被修改。
// 不可变引用：x是对option内部值的不可变引用，所以无法通过x来修改option的值。
