use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionTools, CreateChatCompletionRequestArgs,
};

use crate::tools::calculator::execute::{CalculatorArgs, calculator};

pub async fn chat_tools(
    model: &str,
    system: Option<&str>,
    prompt: &str,
    tools: Vec<ChatCompletionTools>,
) -> anyhow::Result<String> {
    let client = async_openai::Client::new();

    let mut messages = Vec::new();

    if let Some(value) = system {
        messages.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(value)
                .build()?
                .into(),
        );
    }

    messages.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into(),
    );

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages.clone())
        .max_tokens(2048u32)
        .tools(tools.clone())
        .build()?;
    let response = client.chat().create(request).await?;

    tracing::info!("response: {:#?}", response);

    let message = response
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No choices in response"))?
        .message;

    if let Some(tool_calls) = message.tool_calls {
        messages.push(
            ChatCompletionRequestAssistantMessageArgs::default()
                .tool_calls(tool_calls.clone())
                .build()?
                .into(),
        );

        for tool_call in tool_calls {
            match tool_call {
                ChatCompletionMessageToolCalls::Function(function_call) => {
                    let function_name = function_call.function.name;
                    let arguments = function_call.function.arguments;
                    tracing::info!("function_name:{}", function_name);
                    tracing::info!("arguments: {}", arguments);
                    if function_name == "calculator" {
                        let args: CalculatorArgs = serde_json::from_str(&arguments)?;
                        let result =
                            calculator(&args.operator, args.first_number, args.second_number)?;
                        tracing::info!("calculator result:{}", result);
                        messages.push(
                            ChatCompletionRequestToolMessageArgs::default()
                                .tool_call_id(function_call.id.clone())
                                .content(result.to_string())
                                .build()?
                                .into(),
                        );
                    }
                }
                _ => tracing::error!("error"),
            }
        }
        let request = CreateChatCompletionRequestArgs::default()
            .messages(messages.clone())
            .model(model)
            .tools(tools.clone())
            .max_tokens(2048u32)
            .build()?;
        let response = client.chat().create(request).await?;
        let content = response
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .ok_or_else(|| anyhow::anyhow!("No chioce in response"))?;
        return Ok(content);
    }

    let content = message
        .content
        .ok_or_else(|| anyhow::anyhow!("No chioce in response"))?;
    Ok(content)
}
