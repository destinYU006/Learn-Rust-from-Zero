// 引入
fn return_summarizable(switch:bool) -> impl Summary{
  if switch{
    Post{}
  }else{
    Weibo{}
  }
}
//  其中post 和weibo 都实现了Summary 特征，因此上面的函数试图通过返回 impl Summary来返回这两个类型，编译器报错。因为
// impl  Trait 的返回值并不支持多种不同的类型返回。那么如何返回多种不同的类型呢？
// 使用枚举可以实现外面一层包装，但是枚举需要提前知道类型，如果不知道类型呢？其他语言可以用类的继承实现

// rust中使用 特征对象实现
// 01- 特征对象的定义
//     特征对象是指实现了特征的类型的实例。这种映射关系是存储在一张表中，可以在运行时通过特征对象找到具体的调用的类型方法。
// 可以通过&  引用或者Box<T>智能指针的方式创建特征对象

trait Draw {
  fn draw(&self) ->String;
}
impl Draw for u8 {
  fn draw(&self) -> String{
    format!("{u8: {}}",*self)
  }
}
impl Draw for f64 {
  fn draw(&self) -> String{
    format!("{f64: {}}",*self)
  }
}
// 若T 实现了Draw特征，则调用该函数时 传入的Box<T>可以被隐式转换成函数参数签名中的Box<dyn Draw>
fn draw1(x:Box<dyn Draw>) {
  // 由于实现了Deref特征，Box智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型的定义的`draw`方法
  x.draw();
}
fn draw2(x:&dyn Draw) {
  x.draw();
}
fn main(){
  let x =1.1f64;
  let y = 8u8;
  draw1(Box::new(x));
  draw1(Box::new(y));
  draw2(&x);
  draw2(&y);
  
  
  
}
// 通俗易懂的介绍 特征Trait、泛型Generic 和特征对象 TraitObject
// 特征（Trait）：定义 "能力"
// 特征就像是一份 "能力清单"，它规定了实现者必须具备的方法。
// 比如，在乐器演奏会中，所有乐器都必须能 "演奏"（play）和 "调音"（tune）：
// rust
// trait Instrument {
//     fn play(&self);
//     fn tune(&mut self);
// }
// 任何乐器（如钢琴、小提琴、鼓）只要实现了这两个方法，就被认为是一种乐器。

// 泛型（Generic）："模板化" 的函数或类型
// 泛型就像是 "万能工具"，它可以处理多种类型，但要求这些类型必须满足特定条件（实现某个特征）。
// 比如，有一个 "乐器测试员" 函数，它可以测试任何乐器：
// rust
// fn test_instrument<T: Instrument>(instrument: T) {
//     instrument.tune();
//     instrument.play();
// }
// T: Instrument 表示：这里的 T 可以是任何实现了 Instrument 特征的类型。
// 编译器在编译时会为每个具体类型生成专门的代码（静态分派）。

// 特征对象（Trait Object）："运行时多态"
// 特征对象就像是 "黑匣子"，它隐藏了具体类型，只关心对象是否实现了某个特征。
// 比如，有一个 "乐器收藏盒"，它可以存放任何乐器，但只知道它们是乐器：
// rust
// fn add_to_collection(instrument: Box<dyn Instrument>) {
//     // 存放乐器的逻辑...
// }
// Box<dyn Instrument> 是一个特征对象，表示 " 任何实现了 Instrument 特征的类型 "。
// 具体类型在运行时通过 "虚表"（vtable）动态调用方法（动态分派）。

// 三者的关系
// 特征（Trait）是基础：它定义了一组行为规范。
// 泛型（Generic）是编译时的多态：通过特征约束，让函数或类型可以处理多种实现了该特征的类型。
// 特征对象（Trait Object）是运行时的多态：通过隐藏具体类型，让代码可以在运行时处理不同类型的对象。

// 对比表格
// 特性	          泛型（Generic）        	              特征对象（Trait Object）
// 类型可见性	    编译时知道具体类型	                    运行时才知道具体类型
// 分派方式	      静态分派（编译时确定方法）              动态分派（运行时通过虚表查找方法）
// 内存布局	      每个具体类型生成独立代码（零成本抽象）	  统一的胖指针（数据指针 + 虚表指针）
// 灵活性	        类型必须在编译时确定	                  可以在运行时动态处理不同类型
// 使用场景	      代码逻辑相同，但处理多种类型时	          需要存储或传递不同类型的对象时

fn main(){
  let screen =screen{
    components:vec![
      Box::new(SelectBox{
      width:75,
      height:10,
      options:vec![
        String::from("Yes"),
        String::from("Maybe"),
        String::from("No")   
        ],
      }),
      Box::new(Button{
        width:50,
        height:10,
        label:String::from("OK"),
      }),
    ],
  };
  screen.run();
}
// 注意 dyn 不能单独作为特征对象的定义，例如下面的代码编译器会报错，
// 原因是特征对象可以是任意实现了某个特征的类型，编译器在编译期不知道该类型的大小，不同的类型大小是不同的。
// 而 &dyn 和 Box<dyn> 在编译期都是已知大小，所以可以用作特征对象的定义。

// fn draw2(x: dyn Draw) {
//     x.draw();
// }


// 03-特征对象的动态分发

// 04-Self 与 self

// self：方法的实例参数
// self 就像是一个 "快递员"，它代表当前调用方法的 实例对象。
// 当你调用一个对象的方法时，self 会把这个对象 "送" 进方法里。
// 它有三种形式：
// self（获取所有权）：快递员把包裹拿走，原主人不再拥有。
// &self（不可变借用）：快递员只看看包裹，不修改内容。
// &mut self（可变借用）：快递员可以打开包裹并修改内容。

// Self：类型的别名
// Self 就像是一个 "身份证"，它代表当前类型本身。
// 当你在 impl 块中写代码时，Self 可以用来指代 当前实现的类型。
// 常用于返回值类型、关联类型或方法链。

// 一句话总结
// self 是 "快递员"，负责传递和操作实例对象。
// Self 是 "身份证"，代表类型本身，用于创建或引用该类型。
// self 是 "张三本人"，可以做自我介绍、修改年龄。
// Self 是 "人类" 这个物种，可以创建新人类或定义人类共有的特性。

// 05- 特征对象的限制
// 特征对象的限制：为什么有些特征不能成为特征对象？
// 我们可以用一个 "角色扮演游戏" 的例子来理解特征对象的限制。
// 1. 为什么方法不能返回 Self？
// 场景：
// 在游戏中，每个角色都可以 "复制" 自己（类似 Clone 特征）。

// 法师复制后还是法师，战士复制后还是战士。
// 如果我们只知道角色是 "游戏角色"（特征对象），但不知道具体是法师还是战士，就无法正确复制！

// 2. 为什么方法不能有泛型参数？
// 场景：
// 游戏中，有些角色可以 "装备" 不同类型的武器（泛型方法）。

// 法师可以装备法杖（Weapon<Staff>），战士可以装备剑（Weapon<Sword>）。
// 如果只知道角色是 "游戏角色"（特征对象），就无法确定该装备什么武器！

// 对象安全的本质：类型信息的 "遗忘"
// 特征对象就像一个 "黑盒子"，只知道它实现了某些方法，但不知道具体是什么类型。

// 如果方法返回 Self 或使用泛型参数，就必须知道具体类型才能工作。
// 但特征对象故意 "抹去" 了具体类型，导致这些方法无法正常工作。
