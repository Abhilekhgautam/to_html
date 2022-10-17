use crate::HtmlGenerator;
const KEYWORDS: &[&str] = &["int", "main", "std"];
const OPERATORS: &[&str] = &["()", "::", "{\n", "}"];

struct CodeBlock<'a> {
    text: &'a str,
    //styling : style;
}

impl<'a> CodeBlock<'a> {
    fn new(s: &'a str) -> Self {
        if s.starts_with("```") && s.ends_with("```") {
            let stripped_text_front = match s.strip_prefix("```") {
                Some(str) => str,
                None => "",
            };
            let stripped_text = match stripped_text_front.strip_suffix("```") {
                Some(str) => str,
                None => "",
            };

            CodeBlock {
                text: stripped_text,
            }
        } else {
            let stripped_text_front = match s.strip_prefix("`") {
                Some(str) => str,
                None => "",
            };
            let stripped_text = match stripped_text_front.strip_suffix("`") {
                Some(str) => str,
                None => "",
            };

            CodeBlock {
                text: stripped_text,
            }
        }
    }
}
impl <'a>HtmlGenerator for CodeBlock<'a>{
    fn generate_html(&self) -> String{
        let mut text_code = String::new();
        let mut main_code = String::new();
    
        let code: Vec<_> = self.text.split(' ').collect();
        for elm in code {
            if is_keyword(&elm) {
                text_code = format!(r#"<pre class = "keyword">{elm}</pre>"#);
            } else if is_operator(&elm) {
                text_code = format!(r#"<pre class = "operator">{elm}</pre>"#);
            } else {
                text_code = elm.to_string();
            }
            main_code.push_str(text_code.as_str());
            main_code.push_str(" ");
        }
        format!(r#"<div class = "main-container"><pre>{main_code}</pre></div>"#)
    }
}


fn is_keyword(elm: &str) -> bool {
    KEYWORDS.contains(&elm)
}

fn is_operator(elm: &str) -> bool {
    OPERATORS.contains(&elm)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn generates_codeblock() {
        let code = CodeBlock::new(r#"int main(){
            std::cout << "Hello, World";
            }"#);
        assert_eq!(code.generate_html(), r#"<div class = "main-container"><pre><pre class = "keyword">int</pre> main(){\n   std::cout << "Hello, World";\n   <pre class = "operator">}</pre> </pre></div>"#.to_string()

  );
    }
}
