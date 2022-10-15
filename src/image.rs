///This represents img tag in html.
/// The height and width of the Image defaults to 500.
#[derive(PartialEq, Debug)]
struct Image{

  src    : String,
  alt    : Option<String>,
  height : Option<String>,
  width  : Option<String>,


}

fn generate_image(img : Image) -> String {
 let alt = match img.alt{

     Some(str) => str,
     None      => "".to_string()

 };

 let height = match img.height {

   Some(str) => str,
   None      => "500".to_string()

 };


 let width =  match img.width {

    Some(str) => str,
    None      => "500".to_string()

 };

 let src = img.src;

  format!(r#"<img src = "{src}" alt = "{alt}" height = "{height}" width = "{width}"/>"#)

}

#[cfg(test)]
mod tests{

   use super::*;

   #[test]
   fn generates_img() {

     assert_eq!(generate_image(Image {

       src    : String::from("https://abhi-lekhgautam.web.app/pp"),
       alt    : Some("Abhilekh Gautam's profile picture".to_string()),
       height : None,
       width  : None,
     }), r#"<img src = "https://abhi-lekhgautam.web.app/pp" alt = "Abhilekh Gautam's profile picture" height = "500" width = "500"/>"#.to_string());

   }
}
