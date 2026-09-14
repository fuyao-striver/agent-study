use agant_study::{constants::llm::DEEPSEEK_V4_1_FLASH, init, llm::stream::chat_stream};
use futures::StreamExt;
use std::io::Write;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init()?;

    let s = chat_stream(
        DEEPSEEK_V4_1_FLASH,
        Some("你是一个全能助手"),
        "请输出滕王阁序",
    );
    futures::pin_mut!(s);
    let mut output = String::new();
    while let Some(value) = s.next().await {
        match value {
            Ok(text) => {
                output.push_str(&text);
                print!("{}", text);
                // 手动刷新，否则标准输出会攒在缓冲区里，看起来像是“没有输出”
                std::io::stdout().flush()?;
            }
            Err(err) => {
                tracing::error!("\nError while streaming: {}", err);
            }
        }
    }
    println!("\n[共 {} 字符]", output.chars().count());
    Ok(())
}
