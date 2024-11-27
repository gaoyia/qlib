#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use qlib::ThreadPool;

    #[test]
    fn test_thread_pool_execute() {
        // 创建一个包含 4 个线程的线程池
        let mut pool = ThreadPool::new(4);

        // 创建一个共享的计数器
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            });
        }

        // 等待所有任务完成
        pool.join();

        // 检查计数器是否正确
        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 10);
    }

    #[test]
    fn test_thread_pool_join() {
        // 创建一个包含 2 个线程的线程池
        let mut pool = ThreadPool::new(2);

        // 提交一个长时间运行的任务
        pool.execute(|| {
            thread::sleep(Duration::from_secs(2));
        });

        // 提交一个短时间运行的任务
        pool.execute(|| {
            thread::sleep(Duration::from_millis(500));
        });

        // 等待所有任务完成
        pool.join();

        // 检查线程池是否正确等待所有任务完成
        // 这里没有具体的断言，主要是检查程序是否正常结束
    }

    #[test]
    #[should_panic(expected = "Need at least 1 worker!")]
    fn test_thread_pool_zero_workers() {
        // 尝试创建一个包含 0 个线程的线程池，应该会 panic
        ThreadPool::new(0);
    }
}