use crate::HtmlGenerator;
/// Represents <a> of html
/// represented by the keyword directTo:
/// eg: directTo:https://abhi-lekhgautam.web.app, Abhilekh Gautam;
struct Anchor<'a>{
   reference: &'a str,
   text: &'a str,
}

impl <'a>Anchor<'a>{
    fn new(s:&'a str) -> Self{
      let stripped_param = match s.strip_prefix("directTo:"){
        Some(str) => str,
        None            => ""
      };
      let params: Vec<_> = stripped_param.split(",").collect();
      if params.len() != 2{
        panic!("Invalid parameters to directTo:");
      } else{
        Anchor{
            reference: params[0],
            text     : params[1],
        }
      }
    }
}
impl <'a> HtmlGenerator for Anchor<'a>{
  fn generate_html(&self) -> String {
      let reference = self.reference;
      let text      = self.text;
      format!(r#"<a href = "{reference}">{text}</a>"#)
  }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn generates_anchor(){
        let anchor = Anchor::new(r#"directTo:https://abhi-lekhgautam.web.app,Abhilekh Gautam"#);
        assert_eq!(anchor.generate_html(),r#"<a href = "https://abhi-lekhgautam.web.app">Abhilekh Gautam</a>"#);
    }

}