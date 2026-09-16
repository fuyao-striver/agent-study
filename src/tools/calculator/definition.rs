use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools, FunctionObjectArgs};
use serde_json::json;

/// 自定义计算器工具定义
pub fn calculator_tool_definition() -> ChatCompletionTools {
    ChatCompletionTools::Function(ChatCompletionTool {
        function: FunctionObjectArgs::default()
            .name("calculator")
            .description("Perform basic arithmetic operations.")
            .parameters(json!({
                "type": "object",
                "properties": {
                    "operator": {
                        "type": "string",
                        "description": "Arithmetic operation to perform",
                        "enum": ["add","subtract","multiply","divide"]
                    },
                    "first_number": {
                        "type": "number",
                        "description": "First number for the calculation",
                    },
                    "second_number": {
                        "type": "number",
                        "description": "Second number for the calculation",
                    },
                },
                "required": ["operator","first_number","second_number"]
            }))
            .build()
            .expect("Failed to build calculation tool definition"),
    })
}
