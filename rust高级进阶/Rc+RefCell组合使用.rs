use std::cell::RefCell;
use std::rc::Rc;

fn main(){
    let s =Rc::new(RefCell::new("many owners".to_string()));
    let s1= s.clone();
    let s2 = s.clone();
    s2.borrow_mut().push_str(",oh,yeah!");
    println!("{:?}\n{:?}\n{:?}",s,s1,s2);
}
//  s
