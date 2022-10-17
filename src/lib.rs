mod code_block;
mod heading;
mod image;
mod ordered_list;
mod parser;
mod unordered_list;
mod anchor;

use std::fs;

trait HtmlGenerator{
    fn generate_html(&self) -> String;
}

fn main() {
    let file = fs::read_to_string("/home/abhilekh/Downloads/test.txt").unwrap();

    //parse and generate an equivalent html file

    let html_file_path = parser::parse(file).unwrap();
}
