fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");       // "blue" 是 &str 类型
    string("red".to_string());  // to_string() 转为 String
    string(String::from("hi")); // String::from 创建 String
    string("rust is fun!".to_owned()); // to_owned() 转为 String
    string("nice weather".into());     // into() 转为 String
    string(format!("Interpolation {}", "Station")); // format! 返回 String
    string_slice(&String::from("abc")[0..1]); // 切片操作返回 &str
    string_slice("  hello there ".trim());    // trim() 返回 &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // replace 返回 String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // to_lowercase 返回 String
}
