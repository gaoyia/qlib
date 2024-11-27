mod thread_pool;


fn main() {
    // 创建线程池
    let mut pool = thread_pool::ThreadPool::new(10);
    // 执行一些任务
    for i in 0..10 {
        pool.execute(move || {
            println!("Task {} is running", i);
        });
    }
    pool.join();
    print!("任务结束了");
}