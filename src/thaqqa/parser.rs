
use super::database::fonttable::FontTable;
use super::database::datatable::DataType;

use std::collections::HashMap;

pub enum Language{
    Khehyelu,
    Sanghal,
    Raw,
}

pub struct Parser{

}

impl Parser{
    pub fn parse(input:&str,lang:&Language) -> String{
        let regex_datatable =  FontTable::get_datatable_unwraped("REGEX","");

        let mut ans = regex_datatable.get_single_data_from_name_unwraped("Sep_Parser").get_value_as_regex("Parsed").replace_all(input,"R($1)").to_string();
        let mut map = HashMap::new();
        map.insert(String::from("Pattern"), DataType::Regex(regex_datatable.get_single_data_from_name_unwraped(match lang{
            &Language::Khehyelu => "c|u",
            &Language::Sanghal => "o|u",
            _ => "o|u",
        }).get_value_as_regex("Parsed")));

        let mut i = FontTable::get_datatable_unwraped("PARSER","").get_data_unwraped(&map);
        i.sort_by(|a, b| {
            let i_ = a.get_value_as_i32("Order");
            let j_ = b.get_value_as_i32("Order");
            if i_ == j_{
                let i__ = a.get_value_as_i32("Priority");
                let j__ = b.get_value_as_i32("Priority");
                i__.cmp(&j__)
            } else { i_.cmp(&j_) }
        });
        for i_ in i {
            let re = i_.get_value_as_regex("Parsed");
            let alt = i_.get_value_as_string("NameAlt");
            ans = re.replace_all(&ans,alt).to_string();
        }
        ans
    }
}