
/// unordered_list is the html's equivalent of ul tag
pub struct unordered_list {
    list_items: Vec<String>,
}

/// this function generates equivalent html code for a given unordered_list
/// this returns html code as an String
fn generate_unordered_list(list: unordered_list) -> String {
    let mut ul_html = String::from("<ul>");

    for items in list.list_items {
        let mut list_item = String::from("<li>");
        list_item.push_str(items.as_str());
        list_item.push_str("</li>");

        ul_html.push_str(list_item.as_str());
        list_item.clear();
    }
    ul_html.push_str("</ul>");

    ul_html
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generates_unordered_list() {
        assert_eq!(
            generate_unordered_list(unordered_list {
                list_items: vec![
                    "hi".to_string(),
                    "hello".to_string(),
                    "oh thank you".to_string()
                ],
            }),
            String::from("<ul><li>hi</li><li>hello</li><li>oh thank you</li></ul>")
        );
    }
}
