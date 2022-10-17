use crate::HtmlGenerator;
///This represents img tag in html.
/// The height and width of the Image defaults to 500.
#[derive(PartialEq, Debug)]
struct Image<'a> {
    src: &'a str,
    alt: Option<&'a str>,
    height: Option<&'a str>,
    width: Option<&'a str>,
}

impl<'a> Image<'a> {
    fn new(s: &'a str) -> Self {
        let stripped_img_front = match s.strip_prefix("img(") {
            Some(str) => str,
            None => "",
        };
        //stripped_img now contains only the parameters to img
        let stripped_img = match stripped_img_front.strip_suffix(")") {
            Some(str) => str,
            None => "",
        };
        let param: Vec<_> = stripped_img.split(",").collect();

        let param_nos = param.len();

        if param_nos > 4 && param_nos < 1 {
            panic!("Invalid img parameters");
        } else if param_nos == 4 {
            Image {
                src: param[0],
                alt: Some(param[1]),
                height: Some(param[2]),
                width: Some(param[3]),
            }
        } else if param_nos == 3 {
            Image {
                src: param[0],
                alt: Some(param[1]),
                height: Some(param[2]),
                width: None,
            }
        } else if param_nos == 2 {
            Image {
                src: param[0],
                alt: Some(param[1]),
                height: None,
                width: None,
            }
        } else {
            Image {
                src: param[0],
                alt: None,
                height: None,
                width: None,
            }
        }
    }
}

impl <'a>HtmlGenerator for Image<'a>{
    fn generate_html(&self) -> String {
        let alt = match self.alt {
            Some(str) => str,
            None => "",
        };
    
        let height = match self.height {
            Some(str) => str,
            None => "500",
        };
    
        let width = match self.width {
            Some(str) => str,
            None => "500",
        };
    
        let src = self.src;
    
        format!(r#"<img src = "{src}" alt = "{alt}" height = "{height}" width = "{width}"/>"#)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generates_img() {
        let image = Image::new(r#"https://abhi-lekhgautam.web.app/pp, Some("Abhilekh Gautam's profile picture"), None, None"#);
        assert_eq!(image.generate_html(), r#"<img src = "https://abhi-lekhgautam.web.app/pp" alt = "Abhilekh Gautam's profile picture" height = "500" width = "500"/>"#.to_string());
    }
}
