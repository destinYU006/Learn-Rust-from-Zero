#[derive(Debug)]  // 编译器：自动生成 Debug trait 实现，用于打印结构体信息
struct Package {    // 语法：定义结构体，内存中按字段顺序布局
    sender_country: String,  // 语法：String 类型（动态字符串）
                          // 编译器：检查字符串操作安全性
                          // 内存：栈上存储胖指针（24字节），指向堆上字符串数据
    recipient_country: String,  // 同上
    weight_in_grams: i32,       // 语法：32位有符号整数
                          // 编译器：类型检查，内存对齐
                          // 内存：栈上占4字节
}

impl Package {  // 语法：为 Package 结构体实现方法
    // 构造函数，创建 Package 实例
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        // 编译器：类型检查参数和返回值，检查 panic! 宏的有效性
        // 内存：参数通过值传递，触发 String 所有权转移
        if weight_in_grams <= 0 {
            panic!("Can not ship a weightless package.");  // 语法： panic 宏终止程序
                                                         // 内存：释放已分配的栈内存
        } else {
            // 语法：结构体初始化语法（字段名缩写）
            // 内存：在栈上创建 Package 实例，String 数据保留在堆上
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    // 判断是否为国际包裹
    fn is_international(&self) -> bool {  // 语法：借用 self 的不可变引用
                                        // 编译器：确保引用有效性，检查字符串比较
                                        // 内存：通过引用访问字段，不移动数据
        self.sender_country != self.recipient_country  // 语法：字符串内容比较
                                                    // 编译器：生成字符串比较的字节码
                                                    // 内存：比较两个 String 的堆数据
    }

    // 计算运输费用
    fn get_fees(&self, cents_per_gram: i32) -> i32 {  // 语法：参数和返回值均为 i32
                                                    // 编译器：检查整数运算溢出（debug模式）
                                                    // 内存：栈上进行整数乘法运算
        self.weight_in_grams * cents_per_gram  // 语法：整数乘法
                                           // 内存：直接操作栈上的 i32 数据
    }
}
#[cfg(test)]  // 编译器：仅在测试模式下编译此模块
mod tests {    // 语法：定义模块
    use super::*;  // 语法：导入上层模块的内容

    #[test]  // 编译器：标记为测试函数，生成测试框架代码
    #[should_panic]  // 语法：断言函数会 panic
    fn fail_creating_weightless_package() {
        // 内存：在栈上创建 String 胖指针，堆上分配字符串数据
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        // 内存：调用 new 方法时，String 所有权转移到 Package 实例
        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        // 内存：Package 实例在栈上，包含指向堆上 String 的指针
        let package = Package::new(sender_country, recipient_country, 1200);

        // 编译器：检查布尔值断言
        assert!(package.is_international());  // 语法：断言表达式为 true
    }
  

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        // 语法：clone() 方法复制 String 的内容
        // 内存：堆上创建新的字符串数据，栈上复制胖指针
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;  // 内存：栈上存储 i32 数据

        let package = Package::new(sender_country, recipient_country, 1500);

        // 编译器：检查整数相等性断言
        assert_eq!(package.get_fees(cents_per_gram), 4500);  // 语法：断言两值相等
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
