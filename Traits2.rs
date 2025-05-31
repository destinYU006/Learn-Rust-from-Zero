// 定义特征
trait AppendBar {
  fn append_bar(self) -> Self;
}
// 实现AppendBar trait针对Vec<String>类型
impl AppendBar for Vec<String> {
  // 使用`mut self`表示消费并获取向量的所有权，允许修改
  fn append_bar(mut self) ->Self{  //使用Self 意味着后续不能使用动态分发了，只能静态分发
    // 调用Vec的push方法，添加一个新的String元素"Bar"
    // 这里在向量末尾添加元素，可能会触发内存的重新分配
    self.push("Bar".to_string());
    self  // 返回的实例本身及所有权
  }
}
#[cfg(test)]
mod tests{
  use super::*;
  #[test]
  fn is_vec_poip_eq_bar(){
    let mut foo = vec![String::from("Bar")].append_bar();
    assert_eq!(foo.pop().unwrap(),String::from("Bar"));
  }
}
//  01-编译器角度分析
//     类型指定：impl AppendBar for Vec<String> 为字符串实现Trait。rust中没有原生vector类型。
//     方法签名：mut self 正确获取所有权 并允许修改，返回值Self与trait定义严格一致
//     字符串处理："Bar".to_string() 从字符串字面量创建String，等价于String::from("Bar")

//  02-内存操作分析
//     1.所有权：self参数通过值传递，原向量所有权被移动到方法内； 返回self后 所有权转移给调用者
//     2.动态扩容：push 可能会触发Vec的内存重新分配；当容量不足时，Vec通常会按照倍数增长
