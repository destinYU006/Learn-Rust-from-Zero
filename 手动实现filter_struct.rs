// 模拟filter结构体
struct ManualFilter<I, P> {
    iter: I,
    predicate: P,
}

// 为结构体实现方法
impl<I, P> ManualFilter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    fn new(iter: I, predicate: P) -> Self {
        Self { iter, predicate }
    }
}

// 为结构体实现Iterator特征
impl<I, P> Iterator for ManualFilter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(x) => {
                    if (self.predicate)(&x) {
                        return Some(x);
                    }
                }
                None => return None,
            }
        }
    }
}


fn main() {
    let numbers = vec![1, 2, 3, 4];
    let mut evens = Vec::new();
    
    // 修正变量名拼写错误
    let filtered = ManualFilter::new(numbers.iter(), |&x| x % 2 == 0);
    
    for num in filtered {
        evens.push(*num);
    }
    
    println!("Evens: {:?}", evens);
}

//  主要语法点： 
//     1. self 和Self的区别？
//     2. 闭包函数是什么？怎么使用？常用知识点
//     3. 需要从rust语法、编译器、内存操作、性能优化等角度详细分析，逐步绘制内存操作示意图。
//    std::prelude 模块会自动被引入到每个 Rust 程序里，像 Iterator、Option、Result 等常见的 trait 和类型都包含其中
