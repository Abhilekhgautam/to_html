pub struct Parser {
    Expr: Vec<String>,
}

pub fn parse(text: String) -> Result<String, Box<dyn std::error::Error>> {
    let expr: Vec<_> = text.split(";").collect();

    Ok("hello".to_string())
}
