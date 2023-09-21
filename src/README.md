### 内置 actix::Handler
### 枚举匹配

```rust
pub enum MyMessage {
    Login(Login),
    GetTasks(GetTasks),
}
pub trait MessageHandler: Send + Sync {
    fn handle(&self, msg: MyMessage, ctx: &mut ws::WebsocketContext<MyWebSocket>);
}
pub struct MyWebSocket {
    pub id: usize,
    pub handlers: HashMap<String, Box<dyn MessageHandler>>,
}
impl MessageHandler for MyHandler {
    fn handle(&self, msg: MyMessage, ctx: &mut ws::WebsocketContext<MyWebSocket>) {
        match msg {
            MyMessage::Login(login) => {
                // 在这里处理 Login 消息
            },
            MyMessage::GetTasks(get_tasks) => {
                // 在这里处理 GetTasks 消息
            },
            // 在这里处理更多的消息类型
        }
    }
}
```

### Handler 注册表

用HashMap 实现, 例如:
```rust
pub struct MyWebSocket {
    pub id: usize,
    pub handlers: HashMap<String, Box<dyn MessageHandler>>,
}
```


### 把 actix::Hander 放入 HashMap

在Rust中，由于所有权和生命周期的问题，我们不能直接将Handler放入HashMap。但是，我们可以通过使用Arc<Mutex<>>来共享Handler的所有权，然后将这个Arc<Mutex<>>放入HashMap。

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use actix::Handler;

struct MyHandler;

impl Handler<Login> for MyHandler {
    type Result = ();

    fn handle(&mut self, msg: Login, ctx: &mut Self::Context) -> Self::Result {
        // 在这里处理 Login 消息
    }
}

let handler = Arc::new(Mutex::new(MyHandler));
let mut handlers: HashMap<String, Arc<Mutex<dyn Handler<Login, Result=()>>>> = HashMap::new();
handlers.insert("login".to_string(), handler);
```

在这个示例中，我们创建了一个MyHandler实例，并将其封装在Arc<Mutex<>>中。然后，我们创建了一个HashMap，键是字符串，值是Arc<Mutex<dyn Handler<Login, Result=()>>>。最后，我们将handler添加到handlers中。

请注意，由于Handler trait的方法需要可变引用，所以我们需要使用Mutex来在多个线程之间共享可变状态。

>> 我并不需要修改HashMap中的Handler ,只是在此一次初始化包含Handler 的HashMap 后不需要任何修改, 这应该不需要现场安全, 因为HashMap 在创建后就不需要修改了

我并不需要修改HashMap中的Handler ,只是在此一次初始化包含Handler 的HashMap 后不需要任何修改, 这应该不需要现场安全, 因为HashMap 在创建后就不需要修改了

如果你的HashMap在创建后不再需要修改，那么你可以使用Arc而不是Arc<Mutex<>>来共享Handler的所有权。这样，你就可以避免使用互斥锁，同时仍然可以在多个线程之间安全地共享Handler。


```rust
use std::sync::Arc;
use std::collections::HashMap;
use actix::Handler;

struct MyHandler;

impl Handler<Login> for MyHandler {
    type Result = ();

    fn handle(&mut self, msg: Login, ctx: &mut Self::Context) -> Self::Result {
        // 在这里处理 Login 消息
    }
}

let handler = Arc::new(MyHandler);
let mut handlers: HashMap<String, Arc<dyn Handler<Login, Result=()>>> = HashMap::new();
handlers.insert("login".to_string(), handler);
```

在这个示例中，我们创建了一个MyHandler实例，并将其封装在Arc中。然后，我们创建了一个HashMap，键是字符串，值是Arc<dyn Handler<Login, Result=()>>。最后，我们将handler添加到handlers中。

请注意，由于Arc只提供对内部数据的不可变引用，所以你不能在Handler的handle方法中修改Handler的状态。如果你需要修改Handler的状态，你仍然需要使用Arc<Mutex<>>。

>> handlers 在初始化的时候必须是mut, 初始化后有没有办法让他变成不可变的?

在Rust中，一旦变量被声明为可变（mut），就不能再将其更改为不可变。但是，你可以通过创建一个新的不可变引用来实现类似的效果。

例如，你可以这样做：

```rust
let mut handlers: HashMap<String, Arc<dyn Handler<Login, Result=()>>> = HashMap::new();
handlers.insert("login".to_string(), Arc::new(MyHandler));

let handlers = handlers; // 这将创建一个新的不可变引用
```

在这个例子中，我们首先创建了一个可变的handlers变量。然后，我们通过let handlers = handlers;创建了一个新的不可变引用。这个新的引用将覆盖原来的可变引用，所以在这之后，你就不能再修改handlers了。

请注意，这并不会改变原来的handlers变量。原来的handlers变量仍然是可变的，只是你不能再通过这个名字来访问它了。如果你需要再次修改handlers，你需要创建一个新的可变引用。

>> 这样 mut handlers 就被隐藏了? 至少我不能再程序中通过这个名字对其进行修改了?

是的，你是对的。在Rust中，当你使用let handlers = handlers;这样的语句时，你实际上是在创建一个新的不可变引用，并用这个新的引用覆盖了原来的可变引用。这意味着在这之后，你不能再通过handlers这个名字来修改这个变量了。

这是因为在Rust中，变量的可变性是在编译时检查的，而不是在运行时。一旦一个变量被声明为不可变，编译器就会阻止你对这个变量进行任何修改。

所以，虽然原来的handlers变量仍然是可变的，但是你不能再通过这个名字来访问它了。如果你需要再次修改handlers，你需要创建一个新的可变引用。但是，由于你已经创建了一个不可变引用，所以你不能再创建一个新的可变引用，除非你首先删除这个不可变引用。

这就是为什么我们说let handlers = handlers;这样的语句可以用来"隐藏"一个可变引用。
