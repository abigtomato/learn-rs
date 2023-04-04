// 异步运行时
#[cfg(test)]
mod tests {
    
    use std::thread;
    use std::time::Duration;
    use chrono::Local;
    use tokio::{self, runtime::Runtime, time};

    // 要使用tokio，需要先创建它提供的异步运行时环境(Runtime)，然后在这个Runtime中执行异步任务
    #[test]
    fn runtime_test() {
        // 创建runtime
        // 默认情况下(比如以上两种方式)，创建出来的runtime都是多线程runtime，且没有指定工作线程数量时，默认的工作线程数量将和CPU核数(虚拟核，即CPU线程数)相同
        let rt = tokio::runtime::Runtime::new().unwrap();
        println!("{:?}", rt);

        // 创建带有线程池的runtime
        let rt2 = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(8) // 8个工作线程
            .enable_io() // 可在runtime中使用异步IO
            .enable_time() // 可在runtime中使用异步计时器(timer)
            .build() // 创建runtime
            .unwrap();
        println!("{:?}", rt2);

        // 创建单一线程的runtime
        let rt3 = tokio::runtime::Builder::new_current_thread().build().unwrap();
        println!("{:?}", rt3);
    }

    // 可手动创建线程，并在不同线程内创建互相独立的runtime
    // runtime实现了Send和Sync这两个Trait，因此可以将runtime包在Arc里，然后跨线程使用同一个runtime
    #[test]
    fn multi_runtime_test() {
        // 在第一个线程内创建一个多线程的runtime
        let t1 = thread::spawn(|| {
            let rt = Runtime::new().unwrap();
            println!("{:?}", rt);
            thread::sleep(Duration::from_secs(10));
        });

        // 在第二个线程内创建一个多线程的runtime
        let t2 = thread::spawn(|| {
            let rt = Runtime::new().unwrap();
            println!("{:?}", rt);
            thread::sleep(Duration::from_secs(10));
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }

    // 提供了Runtime后，可在Runtime中执行异步任务
    // 每一个异步任务都是一个线程内的【协程】，单一线程的runtime是在单个线程内调度管理这些任务，多线程runtime则是在多个线程内不断地分配和跨线程传递这些任务
    #[test]
    fn async_test() {
        let rt = Runtime::new().unwrap();
        // block_on会阻塞当前线程，直到其指定的异步任务树(可能有子任务)全部完成
        // block_on是等待异步任务完成，而不是等待runtime中的所有任务都完成
        // 直接将async {}作为block_on()的参数，这个async {}本质上是一个Future，即一个异步任务
        let res = rt.block_on(async {
            // 只是定义了Future，此时尚未执行
            // std::time也提供了sleep()，但它会阻塞整个线程，而tokio::time中的sleep()则只是让它所在的任务放弃CPU并进入调度队列等待被唤醒，它不会阻塞任何线程，它所在的线程仍然可被用来执行其它异步任务
            let task = tokio::time::sleep(tokio::time::Duration::from_secs(2));
            // 开始执行task任务，并等待它执行完成
            task.await;
            3
        });
        // block_on也有返回值，其返回值为其所执行异步任务的返回值
        println!("{}", res);
    }

    fn now() -> String {
        Local::now().format("%F %T").to_string()
    }

    // 在runtime外部定义一个异步任务，且该函数返回值不是Future类型
    fn async_task() {
        println!("create an async task: {}", now());
        // 有时候，定义要执行的异步任务时，并未身处runtime内部。例如定义一个异步函数，此时可以使用tokio::spawn()来生成异步任务
        tokio::spawn(async {
            time::sleep(time::Duration::from_secs(10)).await;
            println!("async task over: {}", now());
        });
    }

    fn async_task2(rt: &Runtime) {
        // 除了tokio::spawn()，runtime自身也能spawn，因此，也可以传递runtime(注意，要传递runtime的引用)
        rt.spawn(async {
            time::sleep(time::Duration::from_secs(10)).await;
        });
    }

    async fn async_task3() {
        println!("create an async task3: {}", now());
    }

    #[test]
    fn spawn_test() {
        let rt = Runtime::new().unwrap();
        // 在这个最外层的异步任务内部，还可以创建新的异步任务，它们都将在同一个runtime中执行
        rt.block_on(async {
            // 调用函数，该函数内创建了一个异步任务，将在当前runtime内执行
            async_task();
        });
        rt.block_on(async {
            async_task2(&rt)
        });
        rt.block_on(async_task3());
    }

    #[test]
    fn enter_test() {
        let rt = Runtime::new().unwrap();
        
        // 进入runtime，但不阻塞当前线程
        // block_on()进入runtime时，会阻塞当前线程
        // enter()进入runtime时，不会阻塞当前线程，它会返回一个EnterGuard。EnterGuard没有其它作用，它仅仅只是声明从它开始的所有异步任务都将在runtime上下文中执行，直到删除该EnterGuard
        let guard1 = rt.enter();
        
        // 生成的异步任务并放入当前的runtime上下文中执行
        tokio::spawn(async {
            time::sleep(time::Duration::from_secs(5)).await;
            println!("task1 sleep over: {}", now());
        });
        
        // 释放runtime上下文，这并不会删除runtime
        // 删除EnterGuard并不会删除runtime，只是释放之前的runtime上下文声明
        // 因此，删除EnterGuard之后，可以声明另一个EnterGuard，这可以再次进入runtime的上下文环境
        drop(guard1);

        // 可以再次进入runtime
        let guard2 = rt.enter();
        tokio::spawn(async {
            time::sleep(time::Duration::from_secs(4)).await;
            println!("task2 sleep over: {}", now());
        });

        drop(guard2);

        // 阻塞当前线程，等待异步任务的完成
        thread::sleep(std::time::Duration::from_secs(10));
    }

    // tokio提供了两种功能的线程：
    // 1.用于异步任务的工作线程(worker thread)
    // 2.用于同步任务的阻塞线程(blocking thread)
    #[test]
    fn blocking_test() {
        // 单个线程或多个线程的runtime，指的都是工作线程，即只用于执行异步任务的线程，这些任务主要是IO密集型的任务。tokio默认会将每一个工作线程均匀地绑定到每一个CPU核心上。
        // 有些必要的任务可能会长时间计算而占用线程，甚至任务可能是同步的，它会直接阻塞整个线程(比如thread::time::sleep())，这类任务如果计算时间或阻塞时间较短，勉强可以考虑留在异步队列中，但如果任务计算时间或阻塞时间可能会较长，它们将不适合放在异步队列中，因为它们会破坏异步调度，使得同线程中的其它异步任务处于长时间等待状态，也就是说，这些异步任务可能会被饿很长一段时间
        let rt1 = Runtime::new().unwrap();
        
        // 创建一个blocking thread，可立即执行（由操作系统调度系统决定何时执行）
        // 例如，直接在runtime中执行阻塞线程的操作，由于这类阻塞操作不在tokio系统内，tokio无法识别这类线程阻塞的操作，tokio只能等待该线程阻塞操作的结束，才能重新获得那个线程的管理权。换句话说，worker thread被线程阻塞的时候，它已经脱离了tokio的控制，在一定程度上破坏了tokio的调度系统
        let task = rt1.spawn_blocking(|| {
            println!("in task: {}", now());
            // 注意，是线程的睡眠，不是tokio的睡眠，因此会阻塞整个线程
            thread::sleep(std::time::Duration::from_secs(10));
        });

        // 小睡1毫秒，让上面的blocking thread先运行起来
        std::thread::sleep(std::time::Duration::from_millis(1));
        println!("not blocking: {}", now());

        // 可在runtime内等待blocking_thread的完成
        rt1.block_on(async {
            task.await.unwrap();
            println!("after blocking task: {}", now());
        })
        // tokio允许的blocking thread队列很长(默认512个)，且可以在runtime build时通过max_blocking_threads()配置最大长度。如果超出了最大队列长度，新的任务将放在一个等待队列中进行等待(比如当前已经有512个正在运行的任务，下一个任务将等待，直到有某个blocking thread空闲)
        // blocking thread执行完对应任务后，并不会立即释放，而是继续保持活动状态一段时间，此时它们的状态是空闲状态。当空闲时长超出一定时间后(可在runtime build时通过thread_keep_alive()配置空闲的超时时长)，该空闲线程将被释放。
        // blocking thread有时候是非常友好的，它像独立线程一样，但又和runtime绑定，它不受tokio的调度系统调度，tokio不会把其它任务放进该线程，也不会把该线程内的任务转移到其它线程。换言之，它有机会完完整整地发挥单个线程的全部能力，而不像worker线程一样，可能会被调度器打断。
    }

    #[test]
    fn shutdown_test() {
        let rt = Runtime::new().unwrap();

        // 一个运行5秒的blocking thread，drop rt时，该任务将继续运行，直到自己终止
        rt.spawn_blocking(|| {
            thread::sleep(std::time::Duration::from_secs(5));
            println!("blocking thread task over: {}", now());
        });

        // 进入runtime，并生成一个运行3秒的异步任务，drop rt时，该任务直接被终止
        let _guard = rt.enter();
        rt.spawn(async {
            time::sleep(time::Duration::from_secs(3)).await;
            println!("worker thread task over 1: {}", now());
        });

        // 进入runtime，并生成一个运行4秒的阻塞整个线程的任务，drop rt时，该任务继续运行，直到自己终止
        rt.spawn(async {
            std::thread::sleep(std::time::Duration::from_secs(4));
            println!("worker thread task over 2: {}", now());
        });

        // 先让所有任务运行起来
        std::thread::sleep(std::time::Duration::from_millis(3));

        // 删除runtime句柄
        // drop(rt);

        // tokio提供了另外两个关闭runtime的方式：shutdown_timeout()和shutdown_background()。前者会等待指定的时间，如果正在超时时间内还未完成关闭，将强行终止runtime中的所有线程。后者是立即强行关闭runtime
        rt.shutdown_timeout(std::time::Duration::from_secs(1));
        println!("runtime droped: {}", now());
    }

    #[test]
    fn handle_test() {
        let rt = Runtime::new().unwrap();

        // tokio提供了一个称为runtime Handle的东西，它实际上是runtime的一个引用，可以随意被clone。它可以spawn()生成异步任务，这些异步任务将绑定在其所引用的runtime中，还可以block_on()或enter()进入其所引用的runtime，此外，还可以生成blocking thread
        let handle = rt.handle();
        handle.spawn(async {
            
        });
        handle.spawn_blocking(|| {

        });
        handle.block_on(async {

        });
        let eg = handle.enter();
        drop(eg);
    }
}
