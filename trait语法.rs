//  特征 Trait
//    01- 定义特征：是把一些方法组合在一起，目的是定义一个实现某些目标所必须的行为的集合；
pub trait Summary{
  fn summarize(&self) -> String; // 特征定义时，只定义方法名、输入 和返回变量类型。没有具体的方法实现
}

//    02- 为类型实现特征
pub struct Post {
  pub title: String,
  pub author:String,
  pub content:String,
}
//          为结构体 Post实现 Trait Summary特征
impl Summary for Post {
  fn summarize(&self) ->String {
    format!("文章{}， 作者是{}"，self.tittle,self.author)
  }
}
pub struct Weibo {
  pub username:String,
  pub content: String,
}
impl Summary for Weibo {
  fn summarize(&self) -> String{
    format!("{}发表了微博{}"，self.username,self.content)
  }
}
// 接下来可以在这个类型上调用特征的方法
fn main(){
  let post =Post{title:"rust yuyan".to_string(),author:"sunface".to_string(),content:"very good".to_string()};
  println!("{}",post.summarize());
}

//    03- 特征定义和实现的位置（孤儿原则）
//        默认实现：在特征定义具有默认实现的方法，其他类型无需再实现该方法或者可以选择重写该方法
pub trait Summary {
  fn summarize(&self) -> String{
    String::from("(read more...)")
  }
}
// 04- 使用特征作为函数参数（最常见的高级用法）
pub fn notify(item: &impl Summary) {  // impl Summary 实现了Summary 特征的item参数；
  println!("Breaking news {}",item.summarize());
}
// 05- 特征约束（trait bound）
impl Trait 实际上是语法糖
  pub fn notify<T:Summary>(item:&T) {
  }
pub fn notify(item1:&impl Summary,item2:&impl Summary){}
//  如果需要强制约束两个参数是统一类型呢
pub fn notify<T:Summary>(item1:&T,item2:&T){}
// 泛型类型T 说明item1 item2 必须拥用同样的类型，同时 T:summary 说明 T必须实现了summaryde 特征；
//    052- 多重约束
pub fn notify(item:&(impl Summary + Display)){}
pub fn notify<T: Summary + Display>(item:&T){}
//    053- where 约束 
fn some_function<T:Display +Clone,U:Clone+Debug>(t:&T,u:&U) ->i32{}

fn some_function<T,U>(t:&T,u:&U)->i32
  where T: Display + Clone,
  U: Clone + Debug
  {
    
  }
//     054- 使用特征约束有条件的实现方法和特征
use std::fmt::Display;
struct Pair<T>{
  y:T,
  x:T,
}
// 实现结构体泛型方法
impl<T> Pair<T>{
  fn new(x:T,y:T) ->Self {
    Self {
      x,
      y,
    }
  }
}
impl<T:Display + partialOrd>  Pair<T>{
  fn cmp_display(&self){
    if self.x >= self.y{
      println!("The largest member is x = {}", self.x);
    }else{
      println!("The largest member is y = {}", self.y);
    }
  }
}
