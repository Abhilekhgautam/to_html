const KEYWORDS:&[&str] = &["int", "main", "std"];
const OPERATORS: &[&str] = &["(", ")", "::", "{", "}"];

struct CodeBlock{

  text : Vec<String>,
  //styling : style;
}


fn generate_codeblock(CodeBlock{text}: CodeBlock) -> String{
 let mut text_code = String::new();
 let mut main_code = String::new();
 for elm in text{
  if is_keyword(&elm){
    text_code = format!(r#"<pre class = "keyword">{elm}</pre>"#);
  } else if is_operator(&elm){
    text_code = format!(r#"<pre class = "operator">{elm}</pre>"#);
  } else {
    text_code = elm;
  }
  main_code.push_str(text_code.as_str());
 }
  format!(r#"<div class = "main-container"><pre>{main_code}</pre></div>"#)
}
fn insert_inside_pre_keyword(items: &String) -> String {
   format!(r#"<pre class = "keyword"> {items} </pre>"#)
}
fn insert_inside_pre_operator(items: &String) -> String {
   format!(r#"<pre class  = "operator">"#)
}

fn is_keyword(elm : &str) -> bool {
  KEYWORDS.contains(&elm)
}

fn is_operator(elm: &str) -> bool{
 OPERATORS.contains(&elm)
}
