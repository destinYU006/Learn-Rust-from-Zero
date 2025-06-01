// 定义枚举类型 Message，包含四种变体
// 编译器：为每个变体分配唯一标识，占用内存空间取决于最大变体
// 语法：使用元组结构体语法定义携带数据的变体
// 内存：每个 Message 实例占用 32 字节（最大变体 Move 包含 Point）
enum Message {
    ChangeColor(u8, u8, u8),  // 携带三个 u8 类型参数（RGB 颜色值）
    Echo(String),             // 携带 String 类型参数（动态分配内存）
    Move(Point),              // 携带 Point 结构体（两个 u8 字段）
    Quit,                     // 空变体，仅占枚举标签空间
}

// 定义 Point 结构体，包含两个 u8 类型字段
// 编译器：将结构体布局优化为 2 字节（无需填充）
// 内存：每个 Point 实例占用 2 字节栈空间
struct Point {
    x: u8,
    y: u8,
}

// 定义 State 结构体，包含四个字段
// 编译器：按声明顺序布局字段，可能添加填充字节
// 内存：占用 24 字节（color 3 + position 2 + quit 1 + message 24）
struct State {
    color: (u8, u8, u8),      // 三元组存储 RGB 颜色（3 字节）
    position: Point,         // 存储坐标位置（2 字节）
    quit: bool,              // 存储退出状态（1 字节）
    message: String,         // 存储字符串（24 字节胖指针）
}

// 为 State 结构体实现方法
impl State {
    // 修改颜色字段
    // 编译器：借用 self 的可变引用，确保方法内独占访问
    // 语法：参数类型为元组 (u8, u8, u8)
    // 内存：直接修改栈上的 color 字段值
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    // 设置退出标志
    // 编译器：同上
    // 内存：修改栈上的 quit 字段值
    fn quit(&mut self) {
        self.quit = true;
    }

    // 回显传入的字符串
    // 编译器：消耗传入的 String 所有权，返回新的 String
    // 语法：参数类型为 String，返回类型为 String
    // 内存：传入的 String 被移动到方法内，返回时重新移动所有权
    fn echo(&mut self, s: String) -> String {
        s // 直接返回传入的字符串，不修改 state.message
    }

    // 移动位置
    // 编译器：消耗 Point 所有权，转移到 position 字段
    // 内存：Point 实例从栈上一个位置复制到另一个位置
    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    // 处理消息
    // 编译器：match 表达式必须覆盖所有变体（穷尽性检查），且 所有匹配的类型必须相同。
    // 语法：模式匹配语法，每个分支执行不同操作
    // 内存：根据消息类型执行不同的内存操作
    fn process(&mut self, message: Message) {
        match message {
            // 调用 change_color 方法，传入 RGB 元组
            // 内存：元组值从 Message 实例复制到方法参数
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
            
            // 调用 echo 方法，忽略返回值
            // 编译器：通过块表达式强制返回 ()，解决类型不匹配问题
            // 内存：String 被移动到 echo 方法，返回时被丢弃
            Message::Echo(s) => { let _ = self.echo(s); },
            
            // 调用 move_position 方法，传入 Point 实例
            // 内存：Point 实例从 Message 移动到 position 字段
            Message::Move(p) => self.move_position(p),
            
            // 调用 quit 方法
            // 内存：修改 quit 字段值
            Message::Quit => self.quit(),
        }
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        // 创建 State 实例
        // 内存：在栈上分配 24 字节空间存储 State 实例
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        
        // 处理 ChangeColor 消息
        // 内存：创建临时 Message 实例（32 字节），处理后丢弃
        state.process(Message::ChangeColor(255, 0, 255));
        
        // 处理 Echo 消息
        // 内存：创建 String（堆上分配 11 字节），移动到 Message，处理后丢弃
        state.process(Message::Echo(String::from("hello world")));
        
        // 处理 Move 消息
        // 内存：创建临时 Point 实例（2 字节），移动到 Message，再移动到 state
        state.process(Message::Move(Point { x: 10, y: 15 }));
        
        // 处理 Quit 消息
        // 内存：创建空 Message 实例（仅标签），修改 quit 字段
        state.process(Message::Quit);

        // 断言测试结果
        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}
