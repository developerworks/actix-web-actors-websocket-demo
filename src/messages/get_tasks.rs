// get_tasks.rs

use actix::{Handler, AsyncContext};
use actix_web_actors::ws;
use serde::Deserialize;

use crate::messages::resp::ResponseMessage;

use super::{MessageHandler, MyWebSocket};

// 定义 GetTasks 消息类型
#[derive(Debug,Deserialize)]
pub struct GetTasks;

#[derive(Debug, serde::Serialize)]
pub struct GetTasksResponse {
    tasks: Vec<Task>,
}
#[derive(Debug, serde::Serialize)]
pub struct Task{
    pub id: u32,
    pub name: String
}

impl actix::Message for GetTasks {
    type Result = ();
}

// 实现 GetTasksHandler
#[allow(unused)]
pub struct GetTasksHandler;
#[allow(unused)]
impl MessageHandler for GetTasksHandler {
    fn handle(&self, msg: serde_json::Value, ctx: &mut ws::WebsocketContext<MyWebSocket>) {
        println!("Getting tasks");

        match serde_json::from_value::<GetTasks>(msg) {
            Ok(get_tasks) => {
                let tasks = vec![
                    Task {
                        id: 1,
                        name: "task 1".to_string()
                    },
                    Task {
                        id: 2,
                        name: "task 2".to_string()

                    }
                ];
                let get_tasks_response = GetTasksResponse {
                    tasks
                };

                let response = ResponseMessage {
                    code: "200".to_string(),
                    message: "Login successful".to_string(),
                    data: serde_json::json!(get_tasks_response),
                };
                ctx.address().do_send(response);
            },
            Err(_) => {
                println!("无法将消息转换为 Login 类型");
            }
        }
    }
}

impl Handler<GetTasks> for MyWebSocket {
    type Result = ();

    #[allow(unused_variables)]
    fn handle(&mut self, msg: GetTasks, ctx: &mut Self::Context) -> Self::Result {
        // 在这里处理 Login 消息
    }
}