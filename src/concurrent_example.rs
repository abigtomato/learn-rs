// 并发
#[cfg(test)]
mod tests {

    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    // 由编程语言调用操作系统 API 创建线程的模型有时被称为 1:1，一个 OS 线程对应一个语言线程
    // 编程语言提供的线程被称为 绿色（green）线程，使用绿色线程的语言会在不同数量的 OS 线程的上下文中执行它们。为此，绿色线程模式被称为 M:N 模型：M 个绿色线程对应 N 个 OS 线程，这里 M 和 N 不必相同
    // 绿色线程的 M:N 模型需要更大的语言运行时来管理这些线程。因此，Rust 标准库只提供了 1:1 线程模型实现
    #[test]
    fn new_thread() {
        let v = vec![1, 2, 3];

        // 使用 spawn 创建新线程
        // 使用 move 关键字强制闭包获取其使用值的所有权
        let handle = thread::spawn(move || {
            // Rust 会 推断 如何捕获 v，因为 println! 只需要 v 的引用，闭包尝试借用 v。
            // 然而这有一个问题：Rust 不知道这个新建线程会执行多久，所以无法知晓 v 的引用是否一直有效
            // 所以需要 move 关键字获取 v 的所有权，注：所有权被闭包获取后，主线程就无法使用了
            println!("Here's a vector: {:?}", v);
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // 通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束
        // 阻塞（Blocking） 线程意味着阻止该线程执行工作或退出
        handle.join().unwrap();
    }

    // 一个日益流行的确保安全并发的方式是 消息传递（message passing），这里线程或 actor 通过发送包含数据的消息来相互沟通
    // 这个思想来源于 Go 编程语言文档中 的口号：“不要通过共享内存来通讯；而是通过通讯来共享内存。”
    // Rust 中一个实现消息传递并发的主要工具是 通道（channel）
    // 1. 编程中的通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）
    // 2. 代码中的一部分调用发送者的方法以及希望发送的数据，另一部分则检查接收端收到的消息。当发送者或接收者任一被丢弃时可以认为通道被 关闭（closed）了
    #[test]
    fn message_passing() {
        // 使用 mpsc::channel 函数创建一个新的通道；mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写
        // Rust 标准库实现通道的方式意味着一个通道可以有多个产生值的 发送（sending）端，但只能有一个消费这些值的 接收（receiving）端
        let (tx, rx) = mpsc::channel();

        // 通过克隆发送者来创建多个生产者
        // 在创建新线程之前，我们对通道的发送端调用了 clone 方法。这会给我们一个可以传递给第一个新建线程的发送端句柄。我们会将原始的通道发送端传递给第二个新建线程。这样就会有两个线程，每个线程将向通道的接收端发送不同的消息
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // 将发送端移动到一个新建线程中并发送一个字符串，这样新建线程就可以和主线程通讯了
        thread::spawn(move || {
            let val = String::from("hi");
            // 通道的发送端有一个 send 方法用来获取需要放入通道的值。send 方法返回一个 Result<T, E> 类型，所以如果接收端已经被丢弃了，将没有发送值的目标，所以发送操作会返回错误
            tx.send(val).unwrap();
            // 下面的代码编译不会通过：
            // 1. 一旦将值发送到另一个线程后，那个线程可能会在我们再次使用它之前就将其修改或者丢弃。其他线程对值可能的修改会由于不一致或不存在的数据而导致错误或意外的结果
            // 2. send 函数获取其参数的所有权并移动这个值归接收者所有。这可以防止在发送后再次意外地使用这个值
            // println!("val is {}", val);

            // 发送多个值并观察接收者的等待
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // recv 会阻塞线程执行直到从通道中接收一个值。一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了
        // try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：可以编写一个循环来频繁调用 try_recv，在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查
        let received = rx.recv().unwrap();
        println!("Got: {}", received);

        // 将 rx 当作一个迭代器。对于每一个接收到的值，我们将其打印出来。当通道被关闭时，迭代器也将结束
        for received in rx {
            println!("Got: {}", received);
        }
    }

    // 任何编程语言中的通道都类似于单所有权，因为一旦将一个值传送到通道中，将无法再使用这个值。共享内存类似于多所有权：多个线程可以同时访问相同的内存位置
    // 互斥器（mutex）是 mutual exclusion 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据。为了访问互斥器中的数据，线程首先需要通过获取互斥器的 锁（lock）来表明其希望访问数据。锁是一个作为互斥器一部分的数据结构，它记录谁有数据的排他访问权
    #[test]
    fn mutual_exclusion() {
        // 使用关联函数 new 来创建一个 Mutex<T>
        let m = Mutex::new(5);
        {
            // 使用 lock 方法获取锁，以访问互斥器中的数据。这个调用会阻塞当前线程，直到我们拥有锁为止
            // 如果另一个线程拥有锁，并且那个线程 panic 了，则 lock 调用会失败。在这种情况下，没人能够再获取锁，所以这里选择 unwrap 并在遇到这种情况时使线程 panic
            // 一旦获取了锁，就可以将返回值（在这里是num）视为一个其内部数据的可变引用了
            // Mutex<T> 是一个智能指针。更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针。这个智能指针实现了 Deref 来指向其内部数据；其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁
            let mut num = m.lock().unwrap();
            *num = 6;
            // 在内部作用域结束后，num MutexGuard 的锁会自动释放
        }
        println!("m = {:?}", m);

        // 在线程间共享 Mutex<T>
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            // 这里不能直接通过 move 关键字移动 counter，因为在循环内，且只能移动一次
            // 对 counter 使用 Rc<T> 也不行，因为Rc<T> 并不能安全的在线程间共享。当 Rc<T> 管理引用计数时，它必须在每一个 clone 调用时增加计数，并在每一个克隆被丢弃时减少计数。Rc<T> 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断
            // Arc<T> 是一个类似 Rc<T> 并可以安全的用于并发环境的类型。字母 “a” 代表 原子性（atomic），所以这是一个原子引用计数（atomically reference counted）类型
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                // 因为 counter 是不可变的，不过可以获取其内部值的可变引用；这意味着 Mutex<T> 提供了内部可变性，就像 Cell 系列类型那样
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
