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