#[allow(unused_imports)]
use crate::agent::handler::{AgentHandler, SpecialAgent};

#[ignore]
#[tokio::test]
async fn function_agent_test() {
    let handler = AgentHandler::new(SpecialAgent::IoAgent);

    let prompt = String::from("I need to make a shell file that opens a tmus session");
    let function = &handler
        .special_agent
        .get_functions()
        .as_ref()
        .unwrap()
        // Get commands test
        .get(0)
        .unwrap()
        .to_function();
    let response = handler.agent.function_prompt(&prompt, &function).await;
    assert!(response.is_ok());
}

#[ignore]
#[tokio::test]
async fn prompt_agent_test() {
    let handler = AgentHandler::new(SpecialAgent::ChatAgent);

    let prompt = String::from("Hello chat agent");
    let response = handler.agent.prompt(&prompt).await;
    assert!(response.is_ok());
}
