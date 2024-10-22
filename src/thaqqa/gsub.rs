
use super::database::fonttable::FontTable;
use super::database::datatable::DataType;
use super::glyph::Aligment;
use std::collections::HashMap;

pub struct Gsub{

}

impl Gsub{
    pub fn gsub(input:&str,alig:&Aligment,font:&str) -> String{
        let regex_datatable =  FontTable::get_datatable_unwraped("REGEX","");
        let mut ans = input.to_string();
        let mut map = HashMap::new();
        map.insert(String::from("Pattern"), DataType::Regex(regex_datatable.get_single_data_from_name_unwraped(match alig{
            &Aligment::Vertical => "v|u",
            _ => "h|u",
        }).get_value_as_regex("Parsed")));
        
        
        let table_p = vec!["GSUB","F_GSUB","",font];
        for n in 0..2 {
        let mut i = FontTable::get_datatable_unwraped(table_p[n],table_p[n+2]).get_data_unwraped(&map);
        //
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
        }
        ans
    }
}