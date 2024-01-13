use futures_util::Future;
use std::pin::Pin;

use espionox::{
    environment::{
        agent::memory::{messages::MessageRole, Message},
        dispatch::{Dispatch, EnvListener, EnvMessage},
        errors::DispatchError,
        Environment,
    },
    Agent,
};

#[derive(Debug)]
pub struct Forgetful {
    watched_agent_id: String,
}

impl From<&str> for Forgetful {
    fn from(wa: &str) -> Self {
        let watched_agent_id = wa.to_string();
        Self { watched_agent_id }
    }
}

impl EnvListener for Forgetful {
    fn trigger<'l>(&self, env_message: &'l EnvMessage) -> Option<&'l EnvMessage> {
        match env_message {
            EnvMessage::Response(noti) => {
                if let Some(id) = noti.agent_id() {
                    if id == &self.watched_agent_id {
                        Some(env_message)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn method<'l>(
        &'l mut self,
        _trigger_message: &'l EnvMessage,
        dispatch: &'l mut Dispatch,
    ) -> Pin<Box<dyn Future<Output = Result<(), DispatchError>> + Send + Sync + 'l>> {
        Box::pin(async move {
            let watched_agent = dispatch
                .get_agent_mut(&self.watched_agent_id)
                .expect("Failed to get watched agent");
            watched_agent.cache.reset_to_system_prompt();
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("TESTING_API_KEY").unwrap();
    let mut env = Environment::new(Some("testing"), Some(&api_key));
    let agent = Agent::default();
    let mut jerry_handle = env
        .insert_agent(Some("jerry"), agent.clone())
        .await
        .unwrap();

    let sal = Forgetful::from("jerry");
    env.add_listener(sal).await;
    env.spawn().await.unwrap();
    let message = Message::new(MessageRole::User, "whats up jerry");
    for _ in 0..=5 {
        let _ = jerry_handle
            .request_io_completion(message.clone())
            .await
            .unwrap();
    }
    env.finalize_dispatch().await.unwrap();
    let dispatch = env.dispatch.write().await;

    let jerry = dispatch.get_agent_ref(&jerry_handle.id).unwrap();

    println!("Jerry stack: {:?}", jerry.cache);

    assert_eq!(jerry.cache.len(), 0);
    println!("All asserts passed, forgetful working as expected");
}
