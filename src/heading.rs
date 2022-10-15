#[derive(PartialEq, Debug)]
enum HeadingType{
  H1,
  H2,
  H3,
  H4,
  H5,
  H6,
}

#[derive(PartialEq, Debug)]
pub struct Heading{

  heading_type : HeadingType,
  text         : String

}

fn generate_heading(Heading{heading_type,text} : Heading) -> String {

    match heading_type {

      HeadingType::H1 => format!("<h1> {text} </h1>"),
      HeadingType::H2 => format!("<h2> {text} </h2>"),
      HeadingType::H3 => format!("<h3> {text} </h3>"),
      HeadingType::H4 => format!("<h4> {text} </h4>"),
      HeadingType::H5 => format!("<h5> {text} </h5>"),
      HeadingType::H6 => format!("<h6> {text} </h6>"),
    }
}

#[cfg(test)]
mod tests {

 use super::*;

 #[test]
 fn generates_heading(){

  assert_eq!( generate_heading(Heading {

    heading_type : HeadingType::H1,
    text         : "Hello, World".to_string(),

  }), "<h1> Hello, World </h1>".to_string());


  assert_eq!( generate_heading(Heading {

    heading_type : HeadingType::H2,
    text         : "Hello, World".to_string(),

  }), "<h2> Hello, World </h2>".to_string());


 }

}
