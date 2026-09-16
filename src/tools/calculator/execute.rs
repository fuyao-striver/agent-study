pub fn calculator(operator: &str, first_number: f64, second_number: f64) -> anyhow::Result<f64> {
    match operator {
        "add" => Ok(first_number + second_number),
        "subtract" => Ok(first_number - second_number),
        "multiply" => Ok(first_number * second_number),
        "divide" => {
            if second_number == 0.0 {
                Err(anyhow::anyhow!("Cannot divide by zero"))
            } else {
                Ok(first_number / second_number)
            }
        }
        other => Err(anyhow::anyhow!("Unsupported operator:{other}")),
    }
}

#[derive(serde::Deserialize,Debug)]
pub struct CalculatorArgs {
    pub operator: String,
    pub first_number:f64,
    pub second_number:f64
}
