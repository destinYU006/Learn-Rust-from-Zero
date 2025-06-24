use std::thread;            // 提供线程创建和管理功能
use std::sync::{Arc, Mutex, Condvar};  // 提供多线程同步原语

fn main() {
    // 创建共享数据结构：Arc包裹一个元组，包含互斥锁和条件变量
    // Arc (Atomic Reference Counting): 允许多个线程安全地共享数据
    // Mutex: 保护内部数据，确保同一时间只有一个线程可以访问
    // Condvar: 允许线程等待特定条件发生
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    
    // 克隆Arc引用，使数据可以被多个线程共享
    // 每个Arc实例包含一个指向堆上数据的指针和一个原子引用计数器
    let pair2 = pair.clone();
    
    // 创建一个新线程
    thread::spawn(move || {
        // 从Arc中解引用获取内部的Mutex和Condvar引用
        // &*pair2: 先通过*解引用Arc，再通过&获取引用 ?  不是Arc实现了自动解引用吗？ 为什么使用&pair2会报错？ 
       // 在 Rust 中，Arc<T> 确实实现了 Deref 特征（自动解引用），但这主要作用于方法调用和字段访问时的隐式转换。而在模式匹配（解构）时，需要显式处理指针的解引用。
        let (lock, cvar) = &*pair2;
        
        // 获取互斥锁，这会阻塞直到锁可用
        // lock()返回一个LockResult，unwrap()处理可能的错误
        // 内存角度：获取锁的所有权，确保其他线程无法访问内部数据
        let mut started = lock.lock().unwrap();
        
        println!("changing started");
        
        // 修改互斥锁保护的布尔值
        // 由于Mutex提供内部可变性，即使started是不可变引用，仍可修改其内部值
        *started = true;
        
        // 通知等待在条件变量上的一个线程
        // 此时可能有线程正在wait()中阻塞
        cvar.notify_one();
        
        // 当started离开作用域时，MutexGuard被释放，自动解锁
        // 内存角度：锁状态被修改为未锁定，允许其他线程获取锁
    });
    
    // 主线程继续执行，获取共享数据的引用
    let (lock, cvar) = &*pair;
    
    // 主线程获取互斥锁
    // 注意：如果子线程还未释放锁，这里会阻塞
    let mut started = lock.lock().unwrap();
    
    // 检查条件是否满足
    // 必须使用循环，因为存在虚假唤醒的可能（即使没有notify，wait也可能返回）
    while !*started {
        // 释放锁并进入等待状态
        // wait()接受MutexGuard并返回一个新的MutexGuard
        // 内存角度：释放锁的所有权，将当前线程加入条件变量的等待队列
        // 当收到通知时，线程被唤醒，重新获取锁并继续执行
        started = cvar.wait(started).unwrap();
    }
    
    // 条件满足，继续执行
    println!("started changed");
    
    // 当started离开作用域时，锁被自动释放
}
