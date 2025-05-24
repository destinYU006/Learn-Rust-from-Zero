// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
// 修复1: 使用模式匹配代替unwrap();
// 内存角度：避免了额外的内存分配或者解引用，Option<()> 无论是否为None都只占0字节。
// 编译器角度： if let 更符合rust语言习惯的模式匹配，编译器可以更好的优化；
// 性能角度：消除了unwrap()可能带来的panic检查，减少运行时开销。 
    if let Some(value) = my_option {
        println!("Option has value :{:?}",value);
    } else{
        println!("Option is None");
    }
  // 修复2: 添加缺少的逗号
  // 内存角度：正确的数组定义，6个i32固定大小数组，占24个字节 。
            // 1. 字面量数组创建：01-内存分配，编译器在静态存储区（.rodata段）创建一个包含6个i32值得数组，24个字节。生命周期：数据在程序启动时分配，直到程序结束才释放。
            //     02-初始化：数组元素按照顺序存储为常量值。
            // 2. 引用创建： 内存分配： 在栈上创建一个变量名my_arr ,大小为8字节，类型为&[i32,6]
                        // 初始化：my_arr 存储静态数据组的内存地址，引用本质是一个指针，只想静态存储区的数组。
// 性能分析：数组访问时o(1)操作
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);
// 修复3: 使用clear()方法清空向量
//  clear()保留了向量的容量，避免了重新分配内存的开销；
//  编译器角度: resize(0, _)返回值为()，赋值给my_empty_vec会导致类型不匹配错误
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
// 修复4: 使用正确的变量交换方法
  // 内存角度：swap 通过指针操作直接交换两个变量的内存内容，不需要中间变量。
  // 编译器角度：编译器可以内联swap函数，生成高效的汇编代码。
  // 能角度: 交换操作是O(1)，且避免了两次赋值操作可能带来的额外开销
    std::mem::swap(& mut value_a,& mut value_b);
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {}; value b: {}", value_a, value_b);
}
