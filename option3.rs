Struct Point {
  x: i32,
  y: i32,
}
fn main(){
  let y: Option<Point> = Some(Point {x: 100 ,y:200});
  // 使用引用匹配，避免消耗 y
  match  &y {
    Some(p) => Println!("Co-ordinates are {} ,{}",p.x, p.y),
    _=> panic!("no match!"),
  }
  y;
  
}
