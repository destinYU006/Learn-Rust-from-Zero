fn trim_me(input: &str) -> String {
    // 去除字符串两端的空白字符并转为String
    // 编译器：调用str::trim()方法，返回&str，通过to_string()转为String
    // 语法：链式调用方法，符合Rust字符串操作习惯
    // 内存：trim操作不分配新内存，to_string()会在堆上创建新String
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // 在字符串后拼接" world!"
    // 编译器：检查字符串拼接的类型安全性
    // 语法：使用format!宏进行字符串格式化，自动处理类型转换
    // 内存：format!分配新的堆内存存储拼接后的字符串
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // 将字符串中的"cars"替换为"balloons"
    // 编译器：调用str::replace方法，检查参数类型
    // 语法：replace方法接受旧字符串和新字符串作为参数
    // 内存：创建新String存储替换后的内容，原字符串保持不变
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
