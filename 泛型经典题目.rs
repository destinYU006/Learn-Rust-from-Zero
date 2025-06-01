// 修复错误
struct Array<T, const N: usize> {
    data : [T; N]
}

fn main() {
    let arrays = (
        Array{
            data: [1, 2, 3],
        },
        Array{
            data: [1.0, 2.0, 3.0],
        },
        Array{
            data: [1, 2],
        }
    );
}

//分析错误原因： 在 Rust 中，数组元素必须是相同类型和长度的。原始代码尝试将不同类型和长度的Array实例放入同一数组中，这违反了 Rust 的类型系统规则。
// 方案一： 如果你需要保留原始数据且不改变结构，可以使用元组（有限数量的不同类型）：
    //  let arrays =(Array{data:[1,2,3],},Array{data:[1.0,2.0,3.0],},Array{data:[1,2],});
    // 特点：
    // 元素类型和长度可以不同。
    // 元组长度固定（此处为 3 个元素）。
    // 通过索引访问（如arrays.0）。

// 方案二： 使用特征对象（trait object）存储不同类型
    // 如果需要动态长度和类型的集合，可以定义一个特征并使用Box<dyn Trait>
trait ArrayTrait{
    fn len(&self) -> usize;
}
struct Array<T,const N:usize> {
    data:[T;N],
}
impl<T,const N:usize> ArrayTrait for Array<T,N> {
    fn len(&self) ->usize{
      N
    }
}
fn main(){
  let arrays:Vec<Box<dyn ArrayTrait>> =vec![
      Box::new(Array{data:[1,2,3]}),
      Box::new(Array{data:[1.0,2.0,3.0,]}),
      Box::new(Array{data:[1,2]})  
  ];
  for arr in arrays{
    println!("Array length :{}",arr.len());
  }
}
//  特点：
    // 所有元素必须实现ArrayTrait  ，
    // 动态分配内存（Box）,运行时通过vtable调用方法（虚表指针），
    // 元素类型和长度可以不同；
