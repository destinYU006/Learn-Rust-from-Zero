
fn main() {
    // println!("Hello, world!");
    let post = Post::new("Hello Rust");
    let pending = post.request_review();
    let published = pending.approve();
}

struct Draft;
struct Pending;
struct Published;

struct Post<T> {
    content: String,
    state: T,
}

impl Post<Draft> {
    fn new(content : &str) -> Self {

        Post {
            content: content.to_string(),
            state: Draft,
        }
    }
    
    // 消费self 返回新状态类型
    fn request_review(self) -> Post<Pending> {

        Post {
            content: self.content,
            state: Pending,
        }
    }
}

impl Post<Pending> {
    //  再次转移所有权
    fn approve(self) -> Post<Published> {
  
        Post {
            content: self.content,
            state:Published,
        }
    }
}
