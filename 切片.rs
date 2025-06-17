//从编译器、内存、语法角度分析切片引用
fn parse_config(args:&[String]) -> Config{
  let query = args[1].clone();
  let file_path = args[2].clone();
  Config{query,file_path}
}
Struct Config{
  query:String,
  file_path:String,
}
//  一：语法角度
//  参数类型： args:&[String] 表示传入了一个 不可变引用，指向一个String数组切片（slice）
//             切片是动态大小类型（DST），由2部分组成，-指向底层数组的指针；-长度（元素的个数）。
//  生命周期参数： 虽然没有显式标注，但是rust会自动为引用添加生命周期参数
//   fn parse_config<'a>(args:&'a [String]) ->Config{...}  'a 必须至少和函数调用的作用域一样长。
//  引用 VS 所有权：
//             传入的是对String数组的引用，而非所有权
//             函数内部不能改变原数组（操作），但是可以读取和clone

// 二：内存操作
//    1. 栈内存布局
//       - args 参数在栈上存储一个纸箱堆中String数组的指针，以及数组的长度
//    2. 堆内存操作
//       String的结构：每个String实例在堆上存储实际的字符数据，在栈上存储一个纸箱堆的指针、长度、容量；
//       克隆操作：clone()会创建一个新的String实例。栈上：分配新的指针、长度、容量；堆上：复制；原字符串的字节数据

// 栈内存                          堆内存
// +------------------+           +-------------------+
// | args: &[String]   |--------->| Vec<String>       |
// |  - ptr: *const _  |           |  - ptr: *mut u8   |----+
// |  - len: 3         |           |  - len: 3         |    |
// +------------------+           |  - cap: 3         |    |
//                                +-------------------+    |
//                                | [0]: String        |    |
//                                |  - ptr: *mut u8 ---+---->| "hello"
//                                |  - len: 5          |         (字节数据)
//                                |  - cap: 5          |
//                                | [1]: String        |---->| "world"
//                                |  - ptr: *mut u8 ---+
//                                |  - len: 5          |
//                                |  - cap: 5          |
//                                | [2]: String        |---->| "file.txt"
//                                |  - ptr: *mut u8 ---+
//                                |  - len: 8          |
//                                |  - cap: 8          |
//                                +-------------------+

//  三：编译器角度
//       1. 生命周期检查：确保args的引用生命周期足够长，避免悬垂引用；
//       2. 所有权转移：
//           clone()创建了新的String 实例
//       3. 性能优化：
//           虽然clone()会深拷贝字符串数据，但是在实际应用中，String本身已经是堆分配的，克隆是不可避免的。

