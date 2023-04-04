// web服务器
#[cfg(test)]
mod tests {

    use std::{
        fs,
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        sync::{mpsc, Arc, Mutex},
        thread,
    };

    struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Message>,
    }

    // Job 是一个有着 execute 接收到的闭包类型的 trait 对象的类型别名
    type Job = Box<dyn FnOnce() + Send + 'static>;

    enum Message {
        NewJob(Job),
        Terminate,
    }

    impl ThreadPool {
        // 选择 usize 作为 size 参数的类型，因为我们知道为负的线程数没有意义
        fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            // 这里通道将充当任务队列的作用，execute 将通过 ThreadPool 向其中线程正在寻找工作的 Worker 实例发送任务
            // Rust 所提供的通道实现是多 生产者，单 消费者 的。这意味着不能简单的克隆通道的消费端来解决问题
            // 我们希望通过在所有的 worker 中共享单一 receiver，在线程间分发任务
            let (sender, receiver) = mpsc::channel();

            // 为了在多个线程间共享所有权并允许线程修改其值，需要使用 Arc<Mutex<T>>
            // Arc 使得多个 worker 拥有接收端，而 Mutex 则确保一次只有一个 worker 能从接收端得到任务
            let receiver = Arc::new(Mutex::new(receiver));

            // with_capacity 为 vector 预先分配空间。因为已经知道了 vector 中需要 size 个元素
            // 预先进行分配比仅仅 Vec::new 要稍微有效率一些，因为 Vec::new 随着插入元素而重新改变大小
            // 从通道队列中取出任务涉及到修改 receiver，所以这些线程需要一个能安全的共享和修改 receiver 的方式，否则可能导致竞争状态
            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                // 对于每一个新 worker，克隆 Arc 来增加引用计数，如此这些 worker 就可以共享接收端的所有权了
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            ThreadPool { workers, sender }
        }

        // spawn 使用 FnOnce 作为 F 的 trait bound，最终会将传递给 execute 的参数传给 spawn，处理请求的线程只会执行闭包一次，这也进一步确认了 FnOnce 是我们需要的 trait，这里符合 FnOnce 中 Once 的意思
        // 需要 Send 来将闭包从一个线程转移到另一个线程，而 'static 是因为并不知道线程会执行多久
        // FnOnce trait 仍然需要之后的 ()，因为这里的 FnOnce 代表一个没有参数也没有返回值的闭包。正如函数的定义，返回值类型可以从签名中省略，不过即便没有参数也需要括号
        fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            // 把传递过来的闭包包装成 Box 发送到通道中
            let job = Box::new(f);
            // 调用 send 上的 unwrap，因为发送可能会失败，这可能发生于例如停止了所有线程执行的情况，这意味着接收端停止接收新消息了
            self.sender.send(Message::NewJob(job)).unwrap();
        }
    }

    // 为 ThreadPool 实现 Drop Trait，当线程池被丢弃时，应该 join 所有线程以确保他们完成其操作
    impl Drop for ThreadPool {
        fn drop(&mut self) {
            println!("Sending terminate message to all workers.");

            // 向每个 worker 发送一个 Terminate 消息
            // 为什么发送终止消息要和join操作要分开循环？
            // 1. 如果尝试在同一循环中发送消息并立即 join 线程，则无法保证当前迭代的 worker 是从通道收到终止消息的 worker
            // 2. 想象一下只有两个 worker 的场景。如果在一个单独的循环中遍历每个 worker，在第一次迭代中向通道发出终止消息并对第一个 worker 线程调用 join
            // 3. 如果此时第一个 worker 正忙于处理请求，那么第二个 worker 会收到终止消息并停止。我们会一直等待第一个 worker 结束，不过它永远也不会结束因为第二个线程接收了终止消息
            for _ in &mut self.workers {
                self.sender.send(Message::Terminate).unwrap();
            }

            println!("Shutting down all workers.");

            // 这里使用了 &mut 因为 self 本身是一个可变引用而且也需要能够修改 worker
            for worker in &mut self.workers {
                println!("Shutting down worker {}", worker.id);

                // join 需要获取参数的所有权，worker 中的 thread 需要存放 Option<thread::JoinHandle<()> 而不是直接存放 thread::JoinHandle
                // 如果 Worker 存放的是 Option<thread::JoinHandle<()>，就可以在 Option 上调用 take 方法将值从 Some 成员中移动出来而对 None 成员不做处理
                // 正在运行的 Worker 的 thread 将是 Some 成员值，而当需要清理 worker 时，将 Some 替换为 None，这样 worker 就没有可以运行的线程了
                // Option 上的 take 方法会取出 Some 而留下 None。使用 if let 解构 Some 并得到线程，接着在线程上调用 join。如果 worker 的线程已然是 None，就知道此时这个 worker 已经清理了其线程所以无需做任何操作
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }

    // 实现的行为是创建线程并稍后发送代码，这会在 ThreadPool 和线程间引入一个新数据类型来管理这种新行为。这个数据结构称为 Worker
    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        // spawn 返回 JoinHandle<T>，其中 T 是闭包返回的类型
        // 我们的情况中，传递给线程池的闭包会处理连接并不返回任何值，所以 T 将会是单元类型 ()
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
            let thread = thread::spawn(move || {
                // 需要闭包一直循环，向通道的接收端请求任务，并在得到任务时执行他们
                loop {
                    // 首先在 receiver 上调用了 lock 来获取互斥器，接着 unwrap 在出现任何错误时 panic
                    // 如果互斥器处于一种叫做 被污染（poisoned）的状态时获取锁可能会失败，这可能发生于其他线程在持有锁时 panic 了且没有释放锁
                    // 如果锁定了互斥器，接着调用 recv 从通道中接收 Job。最后的 unwrap 也绕过了一些错误，这可能发生于持有通道发送端的线程停止的情况，类似于如果接收端关闭时 send 方法如何返回 Err 一样
                    // 调用 recv 会阻塞当前线程，所以如果还没有任务，其会等待直到有可用的任务。Mutex<T> 确保一次只有一个 Worker 线程尝试请求任务
                    let message = receiver.lock().unwrap().recv().unwrap();

                    // loop循环的写法可以并发执行job：
                    // 1. 使用 loop 并在循环块之内而不是之外获取锁和任务，lock 方法返回的 MutexGuard 在 let job 语句结束之后立刻就被丢弃了
                    // 2. 这确保了 recv 调用过程中持有锁，而在 job() 调用前锁就被释放了，这就允许并发处理多个请求了。
                    match message {
                        Message::NewJob(job) => {
                            println!("Worker {} got a job; executing.", id);
                            job();
                        }
                        Message::Terminate => {
                            println!("Worker {} was told to terminate.", id);
                            break;
                        }
                    }
                }

                // 下面这种写法无法让job的执行并发起来：
                // 1. Mutex 结构体没有公有 unlock 方法，因为锁的所有权依赖 lock 方法返回的 LockResult<MutexGuard<T>> 中 MutexGuard<T> 的生命周期
                // 2. 这允许借用检查器在编译时确保绝不会在没有持有锁的情况下访问由 Mutex 守护的资源，不过如果没有认真的思考 MutexGuard<T> 的生命周期的话，也可能会导致比预期更久的持有锁
                // 3. 因为 while 表达式中的值 job 在整个块一直处于作用域中，job() 调用的过程中其仍然持有锁，这意味着其他 worker 不能接收任务
                // while let Ok(job) = receiver.lock().unwrap().recv() {
                //     println!("Worker {} got a job; executing.", id);
                //     job();
                // }
            });

            Worker {
                id,
                thread: Some(thread),
            }
        }
    }

    // 处理连接
    fn handle_connection(mut stream: TcpStream) {
        // 在栈上声明一个 buffer 来存放读取到的数据。这里创建了一个 1024 字节的缓冲区
        let mut buffer = [0; 1024];
        // 接着将缓冲区传递给 stream.read ，它会从 TcpStream 中读取字节并放入缓冲区中
        stream.read(&mut buffer).unwrap();
        // 函数名的 “lossy” 部分来源于当其遇到无效的 UTF-8 序列时的行为：它使用 �，U+FFFD REPLACEMENT CHARACTER，来代替无效序列
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        let get = b"GET / HTTP/1.1\r\n";

        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        // 在 response 上调用 as_bytes，因为 stream 的 write 方法获取一个 &[u8] 并直接将这些字节发送给连接
        stream.write(response.as_bytes()).unwrap();
        // flush 会等待并阻塞程序执行直到所有字节都被写入连接中；TcpStream 包含一个内部缓冲区来最小化对底层操作系统的调用
        stream.flush().unwrap();
    }

    // Web 服务器中涉及到的两个主要协议是 超文本传输协议（Hypertext Transfer Protocol，HTTP）和 传输控制协议（Transmission Control Protocol，TCP）
    // 这两者都是 请求-响应（request-response）协议，也就是说，有 客户端（client）来初始化请求，并有 服务端（server）监听请求并向客户端提供响应
    #[test]
    fn webserver_example() {
        // 监听 TCP 连接，这段代码会在地址 127.0.0.1:7878 上监听传入的 TCP 流
        // 这个函数叫做 bind 是因为，在网络领域，连接到监听端口被称为 “绑定到一个端口”（“binding to a port”）
        // bind 函数返回 Result<T, E>，这表明绑定可能会失败，例如，连接 80 端口需要管理员权限（非管理员用户只能监听大于 1024 的端口），所以如果不是管理员尝试连接 80 端口，则会绑定失败。另一个例子是如果运行两个此程序的实例这样会有两个程序监听相同的端口，绑定会失败
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        // 初始化一个容量为4的线程池
        let pool = ThreadPool::new(4);

        // incoming 方法返回一个迭代器，它提供了一系列的流（更准确的说是 TcpStream 类型的流）
        // 流（stream）代表一个客户端和服务端之间打开的连接
        // 连接（connection）代表客户端连接服务端、服务端生成响应以及服务端关闭连接的全部请求 / 响应过程
        // for 循环会依次处理每个连接并产生一系列的流供我们处理
        for stream in listener.incoming() {
            // 当客户端连接到服务端时 incoming 方法返回错误是可能的，因为我们实际上没有遍历连接，而是遍历 连接尝试（connection attempts）。连接可能会因为很多原因不能成功，大部分是操作系统相关的。例如，很多系统限制同时打开的连接数；新连接尝试产生错误，直到一些打开的连接关闭为止
            let stream = stream.unwrap();
            // 提交任务到池中
            pool.execute(|| handle_connection(stream));
        }
        println!("Shutting down.");
        // 当 ThreadPool 在 webserver_example 的结尾离开作用域时，其 Drop 实现开始工作，线程池通知所有线程终止
    }
}
