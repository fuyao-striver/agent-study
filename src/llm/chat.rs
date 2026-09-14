use async_openai::types::chat::{ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};

pub async fn chat(model:&str,system:Option<&str>,prompt:&str)->anyhow::Result<String> {
    let client = async_openai::Client::new();

    let mut message = Vec::new();

    if let Some(value) = system {
        message.push(ChatCompletionRequestSystemMessageArgs::default().content(value).build()?.into());
    }
    message.push(ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?.into());

    let request = CreateChatCompletionRequestArgs::default().model(model)
        .messages(message).max_tokens(2048u32).build()?;

    let response = client.chat().create(request).await?;

    tracing::info!("response: {:#?}",response);

    let content = response.choices.into_iter().next().and_then(|choice|choice.message.content)
        .ok_or_else(||anyhow::anyhow!("没有内容!"))?;
    
    Ok(content)
}