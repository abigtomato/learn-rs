// 异步任务
#[cfg(test)]
mod tests {

    use chrono::Local;
    use std::{thread};
    use tokio::{self, task, runtime::Runtime, runtime::Handle, time};

    fn now() -> String {
        Local::now().format("%F %T").to_string()
    }

    #[test]
    fn spawn_test() {
        let rt = Runtime::new().unwrap();
        let _guard = rt.enter();
        // 直接在当前的runtime中生成一个异步任务
        task::spawn(async {
            time::sleep(time::Duration::from_secs(3)).await;
            println!("task over: {}", now());
        });
        thread::sleep(time::Duration::from_secs(4));
    }

    #[test]
    fn spawn_blocking_test() {
        let rt = Runtime::new().unwrap();
        let _guard = rt.enter();
        // 直接在当前的runtime中生成一个异步任务
        let join = task::spawn_blocking(|| {
            println!("blocking completed");
            "blocking completed"
        });
        rt.spawn(async {
            let result = join.await.unwrap();
            print!("{}", result);
            assert_eq!(result, "blocking completed");
        });
        thread::sleep(time::Duration::from_secs(4));
    }

    #[test]
    fn block_in_place() {
        // block_in_place()的目的和spawn_blocking()类似。区别在于spawn_blocking()会新生成一个blocking thread来执行指定的任务，而block_in_place()是在当前worker thread中执行指定的可能会长时间运行或长时间阻塞线程的任务，但是它会先将该worker thread中已经存在的异步任务转移到其它worker thread，使得这些异步任务不会被饥饿
        task::block_in_place(move || {
            Handle::current().block_on(async move {
                // do something async
                println!("");
            });
        });
    }

    #[test]
    fn yield_now_test() {
        let rt = Runtime::new().unwrap();
        rt.spawn(async {
            task::spawn(async {
                println!("spawned task done!");
            });
            // 让当前任务立即放弃CPU，将worker thread交还给调度器，任务自身则进入调度器的就绪队列等待下次被轮询调度
            // 调用yield_now()后还需await才立即放弃CPU，因为yield_now本身是一个异步任务
            // yield后，任务调度的顺序是未知的。有可能任务在发出yield后，紧跟着的下一轮调度会再次调度该任务
            task::yield_now().await;
            println!("main task done!");
        });
        thread::sleep(time::Duration::from_secs(5));
    }
}