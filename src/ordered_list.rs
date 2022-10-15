/// unordered_list is the html's equivalent of ol tag
pub struct OrderedList{

  list_item : Vec<String>,

}

/// this function generates equivalent html code for a given ordered_list
/// this returns html code as an String
fn generate_ordered_list (OrderedList{list_item}: OrderedList) -> String{

  let list_item = list_item.into_iter()
      .map(|item|{
  
           format!("<li>{item}</li>")

      }).collect::<String>();

    format!("<ol>{list_item}</ol>")

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generates_ordered_list() {
        assert_eq!(
            generate_ordered_list(OrderedList {
                list_item: vec![
                    "hi".to_string(),
                    "hello".to_string(),
                    "oh thank you".to_string()
                ],
            }),
            String::from("<ol><li>hi</li><li>hello</li><li>oh thank you</li></ol>")
        );
    }
}
