/// unordered_list is the html's equivalent of ul tag
pub struct UnorderedList {
    list_items: Vec<String>,
}

/// this function generates equivalent html code for a given unordered_list
/// this returns html code as an String
fn generate_unordered_list(UnorderedList { list_items }: UnorderedList) -> String {
    let list_item = list_items
        .into_iter()
        .map(|item| format!("<li>{item}</li>"))
        .collect::<String>();

    format!("<ul>{list_item}</ul>")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generates_unordered_list() {
        assert_eq!(
            generate_unordered_list(UnorderedList {
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
