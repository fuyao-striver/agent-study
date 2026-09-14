use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_stream::stream;
use futures::{Stream, StreamExt};

/// 通过流的方式获取输出
pub fn chat_stream(
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> impl Stream<Item = anyhow::Result<String>> {
    stream! {
    // 1. 创建客户端
    let client = async_openai::Client::new();

    // 2. 构建请求消息
    let mut message = Vec::new();

    if let Some(value) = system {
        message.push(ChatCompletionRequestSystemMessageArgs::default().content(value).build()?.into());
    }
    message.push(ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?.into());

    // 3. 构建请求
    // 注意：deepseek-flash 是推理模型，思考（reasoning_content）也计入 max_tokens。
    // 上限太小会被思考用光，出现 finish_reason=Length 且没有任何可见内容的情况。
    let request = CreateChatCompletionRequestArgs::default().model(model)
        .messages(message).max_tokens(8192u32).build()?;

    // 4. 获得相应流
    let mut stream = client.chat().create_stream(request).await?;

    // 5.相应输出
    while let Some(response) = stream.next().await {
        match response {
            Ok(chunk) => {
                if let Some(chioce) = chunk.choices.first() && let Some(next_txt) = &chioce.delta.content {
                    yield Ok(next_txt.clone())
                }
            },
            Err(err) => yield Err(err.into())
        }
    }
    }
}
