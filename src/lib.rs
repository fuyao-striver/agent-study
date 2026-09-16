use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub mod llm;
pub mod constants;
pub mod tools;

/// 初始化配置
/// 加载环境变量和初始化日志系统
pub fn init() -> anyhow::Result<()>{
    // 读取环境变量
    dotenvy::dotenv()?;
    // 初始化日志
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}