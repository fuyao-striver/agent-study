use agant_study::{
    constants::llm::DEEPSEEK_V4_1_FLASH, init, llm::chat_tools::chat_tools, tools::tools,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init()?;
    let tools = tools();
    let content = chat_tools(
        DEEPSEEK_V4_1_FLASH,
        Some("你是全能助手"),
        "美国有首都的概念吗？",
        tools.clone(),
    )
    .await?;
    println!("{}", content);
    println!("-------------------------------------");
    let content = chat_tools(
        DEEPSEEK_V4_1_FLASH,
        Some("你是全能助手"),
        "11111乘以11111是多少？",
        tools.clone(),
    )
    .await?;
    println!("{}", content);
    Ok(())
}
