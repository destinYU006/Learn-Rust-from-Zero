// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move {x:i32,y:i32},   //  结构体式变体，包含x和y坐标
    Echo(String),         //  携带字符串数据的变体
    ChangeColor(u8,u8,u8),// 元组式变体，包含RGB颜色值

    Quit,                 // 无参数的简单变体
    }
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        // messages 的类型是 [Message; 4]（包含 4 个 Message 枚举的固定大小数组）。
        // &messages 的类型是 &[Message; 4]（指向数组的不可变引用）。
        // 当遍历 &messages 时，Rust 会自动解引用为 &Message 类型的迭代器，
        // 因此 message 的类型是 &Message
        message.call();
    }
}

