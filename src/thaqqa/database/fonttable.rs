use super::datatable::DataTable;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Font{
    name:String,
    table_list:Vec::<DataTable>
}

impl Font{
    pub fn new(name:&str) -> Font{
        Font{
            name:name.to_string(),
            table_list : Vec::new(),
        }
    }


    fn rename(&mut self,name:&str){
        self.name = name.to_string();
    }
    pub fn get_name<'a>(&'a mut self) -> &'a String{
        &self.name
    }
    //unsafe多め
    pub fn get_datatable<'a>(&'a mut self,name:&str) -> Option<&'a mut DataTable>{
        for i in &mut self.table_list{
            if i.name == name {return Some(i);}
        }
        return None;
    }

    pub fn get_datatable_unwraped<'a>(&'a mut self,name:&str) -> &'a mut DataTable{
        if let Some(f) = self.get_datatable(name){
            f
        } else { panic!("Datatable {} が存在しません",name)}
    }

    pub fn create_datatable<'a>(&'a mut self,name:&str) -> Result<&'a mut DataTable,FontError>{
        if self.get_datatable(name).is_some(){
            return Err(FontError::new("既にそのDatatableが存在します"))
        } else {
            let creation = DataTable::new(name);
            let _ = &self.table_list.push(creation);
            return Ok(self.get_datatable(name).unwrap());
        }
    }
    /*
    pub fn rename_datatable<'a>(&'a mut self,name:&str,name_alt:&str) -> Result<&'a mut DataTable,FontError>{
        {//二重借用の回避用
            if self.get_datatable(name_alt).is_some(){
                return Err(FontError::new("名称が重複します"))
            }
        }
        let subject = self.get_datatable(name);
        if subject.is_none(){
            return Err(FontError::new("対象が存在しません"));
        }
         else {
            let subject = subject.unwrap();
            let _ = &mut subject.rename(name); 
            return Ok(subject);
        }
    }

    pub unsafe fn delete_datatable(&mut self,name:&str) -> Result<bool,FontError>{
        let deletion = self.table_list.iter().position(|i| &i.name == name);
        if deletion.is_none(){
            return Err(FontError::new("対象が存在しません"));
        }
        let n = deletion.unwrap();
        let _ = self.table_list.remove(n);
        return Ok(true);//何返せばいいんだ
    }*/

}

#[derive(Debug)]
pub struct FontError {
    /// エラーメッセージ
    pub msg: String,
}

impl FontError {
    fn new(msg: &str) -> FontError {
        FontError {
            msg: msg.to_string(),
        }
    }
}


#[derive(Debug)]
pub struct FontTable{
    font_list:Vec::<Font>,
    statictable_list:Vec::<DataTable>,
}

static mut FT : FontTable = FontTable{
    font_list:Vec::new(),
    statictable_list:Vec::new(),
};

impl Drop for FontTable{
    fn drop(&mut self) {
        unsafe {
            println!("FontTable has been droped.");
            let _ = &FT.font_list.clear();
            let _ = &FT.statictable_list.clear();
        }
    }
}

impl FontTable{
    pub fn load(json:serde_json::Value){

        let obj = json.as_object().unwrap();
        
        for (key,value) in obj.iter() {
            let k = FontTable::create_datatable(key).unwrap();
            k.load(value.clone());
        }
    }
    pub fn load_font(json:serde_json::Value,font:&str){
        let f = FontTable::create_font(font).unwrap();
        let obj = json.as_object().unwrap();
        
        for (key,value) in obj.iter() {
            let k = f.create_datatable(key).unwrap();
            k.load(value.clone());
        }
    }
    //unsafe多め
    /*pub fn get() -> &'static FontTable{
        unsafe{
            &FT
        }
    }*/

    pub fn get_font(name:&str) -> Option<&'static mut Font>{
        unsafe{
            for i in &mut FT.font_list{
                if i.name == name {return Some(i);}
            }
        }
        return None;
    }
    pub fn get_font_unwraped(name:&str) -> &'static mut Font{
        if let Some(f) = FontTable::get_font(name){
            f
        } else { panic!("Font {} が存在しません",name)}
    }
    pub fn create_font(name:&str) -> Result<&'static mut Font,FontError>{
        if FontTable::get_font(name).is_some(){
            return Err(FontError::new("既にそのFontが存在します"))
        } else {
            unsafe {
                let creation = Font{
                    name:name.to_string(),
                    table_list : Vec::new(),
                };
                FT.font_list.push(creation);
                return Ok(FontTable::get_font_unwraped(name));
            }
        }
    }

    pub fn get_datatable(name:&str,font:&str) -> Option<&'static mut DataTable>{
        unsafe{
            for i in &mut FT.statictable_list{
                if i.name == name {return Some(i);}
            }
        }
        if font == "" {return None}
        let font = FontTable::get_font_unwraped(font);
        return font.get_datatable(name);
    }
    pub fn get_datatable_unwraped(name:&str,font:&str) -> &'static mut DataTable{
        if let Some(f) = FontTable::get_datatable(name,font){
            f
        } else { panic!("Datatable {} が存在しません",name)}
    }

    pub fn create_datatable(name:&str) -> Result<&'static mut DataTable,FontError>{
        if FontTable::get_datatable(name,"").is_some(){
            return Err(FontError::new("既にそのDataTableが存在します"))
        } else {
            unsafe {
                let creation = DataTable::new(name);
                FT.statictable_list.push(creation);
                return Ok(FontTable::get_datatable_unwraped(name,""));
            }
        }
    }

}

