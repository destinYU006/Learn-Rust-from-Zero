```rust
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(*x);     // 完成该行代码，不要修改其它行！
// box是实现了deref的，Rust 的自动解引用只会在类型明确不匹配但可以通过解引用修复时触发，而Box<Box<T>>和Box<T>是完全不同的类型，无法通过单次解引用转换 因此不能使用Box::new(x);
    
    *y = 4;
    
    assert_eq!(*x, 5);
}
```
