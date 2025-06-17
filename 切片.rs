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
