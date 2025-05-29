// 泛型的定义、存在原因 初步学习使用语法
// 01-函数泛型
    //  fn largest<T：std::cmp::PartialOrd>(list: &[T]) -> T {}
    //  注意执行比较的话，泛型T必须实现了可以比较的Trait 
//02- 显式指定泛型的类型参数
use std::fmt::Display;
fn create_and_print<T>() where T: From<i32> + Display{
  let a: T = 100.into(); // 创建了类型为T的变量a，它的初始值由100转换而来
  println!("a is :{}",a);
  
}
fn main(){
  create_and_print();
  // 如果直接运行上面代码会报错，因为编译器无法进行单态化处理；
  create_and_print::<i64>();
}

// 03- 结构体中使用泛型
struct Point<T>{
  x:T,
  y:T,
}
fn main(){
  let integet = Point{ x:5,y:10};
}

// 04- 枚举中使用泛型
enum Option<T>{
  Some(T),
  None,
}
enum Result<T,E>{
  Ok(T),
  Err(E),
}

//  05-方法中使用泛型
struct Point<T>{
  x:T,
  y:T，
}

impl<T> Point<T>{
  fn x(&self) -> &T {
    &self.x
  }
}

// 06- const 泛型
fn display_array<T:std::fmt::Debug>(arr: &[T]){
  println!("{:?}",arr);
}
fn display_array<T:std::fmt::Debug,const N:usize>(arr:[T;N]){
  println!("{:?}",arr);
  
}
// 07- const 泛型表达式
fn something<T>(val:T) 
where Assert<{core::mem::size_of::<T>{} < 768}> :IsTrue,
  {
    
  }
// 08- const fn 常量函数 编译期间被调用和执行
//  const fn 的限制：不可以将随机数生成器写成const fn
const fn add(a:usize b:usize) -usize{
  
}
// 09- 结合 const fn 和const 泛型 可以实现更加灵活 高效的代码设计，创建一个固定大小的缓冲区结构，其缓冲器带下在编译期间计算确定
struct Buffer<const N:usize>{
  data:[u8,N],
}
const fn compute_buffer_size(factor:uszie) ->usize{
  factor*1024
}
fn main(){
const SIZE: usize =compute_buffer_size(4);
let buffer =Buffer::<SIZE>{
  data:[0,SIZE],
};
println!("{}",buffer.data.len());
}

