// 面向对象2
#[cfg(test)]
mod tests {

    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        // 新建一个草案状态的实例
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        // 存放博文内容的文本
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        // 返回博文内容
        pub fn content(&self) -> &str {
            // 这里调用 Option 的 as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权。
            // 因为 state 是一个 Option<Box<State>>，调用 as_ref 会返回一个 Option<&Box<State>>。
            // 如果不调用 as_ref，将会得到一个错误，因为不能将 state 移动出借用的 &self 函数参数
            // 接着就有了一个 &Box<State>，当调用其 content 时，解引用强制转换会作用于 & 和 Box ，这样最终会调用实现了 State trait 的类型的 content 方法
            self.state.as_ref().unwrap().content(self)
        }

        // 请求审核博文来改变其状态
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                // 消费当前的状态并返回一个新状态：
                // 1. request_review 方法需要获取状态值的所有权，调用 take 方法将 state 字段中的 Some 值取出并留下一个 None，因为 Rust 不允许结构体实例中存在值为空的字段。这使得我们将 state 的值移出 Post 而不是借用它
                // 2. 需要将 state 临时设置为 None 来获取 state 值，即老状态的所有权，而不是使用 self.state = self.state.request_review(); 这样的代码直接更新状态值。这确保了当 Post 被转换为新状态后不能再使用老 state 值
                self.state = Some(s.request_review())
            }
        }

        // 审核通过改变状态
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    // State trait 定义了所有不同状态的博文所共享的行为
    trait State {
        // 不同于使用 self、 &self 或者 &mut self 作为方法的第一个参数，这里使用了 self: Box<Self>
        // 这个语法意味着该方法只可在持有这个类型的 Box 上被调用。这个语法获取了 Box<Self> 的所有权使老状态无效化，以便 Post 的状态值可转换为一个新状态
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        // 这里获取 post 的引用作为参数，并返回 post 一部分的引用，所以返回的引用的生命周期与 post 参数相关
        fn content<'a>(&self, _: &'a Post) -> &'a str {
            // 默认实现来返回一个空字符串 slice
            ""
        }
    }

    struct Draft {}

    impl State for Draft {
        // 返回一个新的，装箱的 PendingReview 结构体的实例，其用来代表博文处于等待审核状态
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        // 不进行任何状态转换。相反它返回自身，因为当我们请求审核一个已经处于 PendingReview 状态的博文，它应该继续保持 PendingReview 状态
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        // 状态转换为 Published
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    // 状态模式（state pattern）是一个面向对象设计模式。该模式的关键在于一个值有某些内部状态，体现为一系列的 状态对象，同时值的行为随着其内部状态而改变
    // 状态模式的优势：无论 state 是何值，Post 的 request_review 方法都是一样的。每个状态只负责它自己的规则
    // 实现一个增量式的发布博文的工作流。这个博客的最终功能看起来像这样:
    // 1. 博文从空白的草案开始。
    // 2. 一旦草案完成，请求审核博文。
    // 3. 一旦博文过审，它将被发表。
    // 4. 只有被发表的博文的内容会被打印，这样就不会意外打印出没有被审核的博文的文本。
    #[test]
    fn oop_test() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
