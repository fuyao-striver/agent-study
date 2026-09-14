use agant_study::{constants::llm::DEEPSEEK_V4_1_FLASH, init, llm::chat::chat};

#[tokio::main]
async fn main()->anyhow::Result<()> {
    init()?;
    let content = chat(DEEPSEEK_V4_1_FLASH, Some("你是全能助手"),"美国有首都的概念吗？").await?;
    println!("{}",content);
    Ok(())
}