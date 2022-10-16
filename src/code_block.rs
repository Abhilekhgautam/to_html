const KEYWORDS: &[&str] = &["int", "main", "std"];
const OPERATORS: &[&str] = &["()", "::", "{\n", "}"];

struct CodeBlock {
    text: String,
    //styling : style;
}

fn generate_codeblock(CodeBlock { text }: CodeBlock) -> String {
    let mut text_code = String::new();
    let mut main_code = String::new();

    let code: Vec<_> = text.split(' ').collect();
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
fn insert_inside_pre_keyword(items: &String) -> String {
    format!(r#"<pre class = "keyword"> {items} </pre>"#)
}
fn insert_inside_pre_operator(items: &String) -> String {
    format!(r#"<pre class  = "operator">"#)
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
        assert_eq!(generate_codeblock(CodeBlock{

      text: r#"int main(){
   std::cout << "Hello, World";
   }"#.to_string()

    }), r#"<div class = "main-container"><pre><pre class = "keyword">int</pre> main(){\n   std::cout << "Hello, World";\n   <pre class = "operator">}</pre> </pre></div>"#.to_string()

  );
    }
}
