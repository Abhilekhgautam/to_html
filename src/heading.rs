use crate::HtmlGenerator;
#[derive(PartialEq, Debug)]
enum HeadingType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(PartialEq, Debug)]
pub struct Heading<'a> {
    heading_type: HeadingType,
    text: &'a str,
    id: &'a str,
}

impl<'a> Heading<'a> {
    fn new(s: &'a str) -> Self {
        let mut heading_type = HeadingType::H1;
        let mut text = "";
        if s.starts_with("######") {
            heading_type = HeadingType::H6;
            text = match s.strip_prefix("######") {
                Some(txt) => txt,
                None => " ",
            };
        } else if s.starts_with("#####") {
            heading_type = HeadingType::H5;
            text = match s.strip_prefix("#####") {
                Some(txt) => txt,
                None => " ",
            };
        } else if s.starts_with("####") {
            heading_type = HeadingType::H4;
            text = match s.strip_prefix("####") {
                Some(txt) => txt,
                None => " ",
            };
        } else if s.starts_with("###") {
            heading_type = HeadingType::H3;
            text = match s.strip_prefix("###") {
                Some(txt) => txt,
                None => " ",
            };
        } else if s.starts_with("##") {
            heading_type = HeadingType::H2;
            text = match s.strip_prefix("##") {
                Some(txt) => txt,
                None => " ",
            };
        } else {
            text = match s.strip_prefix("#") {
                Some(txt) => txt,
                None => " ",
            };
        }
        let id = text.split(",").collect::<Vec<_>>().into_iter().map(|item| {
            match item.strip_prefix("id:"){
            Some(str) => str,
            None      => "",
    }}).collect::<Vec<_>>();
    if id.len() != 1{
        panic!("Invalid heading");
    }
        Heading { heading_type, text, id:id[0] }
    }

}

impl<'a> HtmlGenerator for Heading<'a>{
    fn generate_html(&self) -> String {
        let text = self.text;
        let id = self.id;
        if id.is_empty() {
            match self.heading_type {
                HeadingType::H1 => format!("<h1> {text} </h1>"),
                HeadingType::H2 => format!("<h2> {text} </h2>"),
                HeadingType::H3 => format!("<h3> {text} </h3>"),
                HeadingType::H4 => format!("<h4> {text} </h4>"),
                HeadingType::H5 => format!("<h5> {text} </h5>"),
                HeadingType::H6 => format!("<h6> {text} </h6>"),
            }
        } else{
            match self.heading_type {
                HeadingType::H1 => format!(r#"<h1 id = "{id}"> {text} </h1>"#),
                HeadingType::H2 => format!(r#""<h2 id = "{id}"> {text} </h2>"#),
                HeadingType::H3 => format!(r#""<h3 id = "{id}"> {text} </h3>"#),
                HeadingType::H4 => format!(r#""<h4 id = "{id}"> {text} </h4>"#),
                HeadingType::H5 => format!(r#""<h5 id = "{id}"> {text} </h5>"#),
                HeadingType::H6 => format!(r#""<h6 id = "{id}"> {text} </h6>"#),
            }
        }
        
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generates_heading() {
        let heading = Heading::new("# Hello, World");
        assert_eq!(
            heading.generate_html(),
            "<h1> Hello, World </h1>".to_string()
        );
    }
}
