use std::collections::HashMap;
use super::super::svg::svg::SVG;
extern crate regex;
use regex::Regex;

#[derive(Debug,Clone)]
pub enum DataType{
    Char(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    Regex(Regex),
    Svg(SVG),
}

impl DataType{
    pub fn is_equal(&self,other:&DataType) -> bool{
        match self{
            DataType::Char(v0) => {
                match other{
                    DataType::Char(v1) => {
                        v0 == v1
                },
                _ => false
                }
            },
            DataType::Int(v0) => {
                match other{
                    DataType::Int(v1) => {
                        v0 == v1
                },
                _ => false
                }
            },
            DataType::Float(v0) => {
                match other{
                    DataType::Float(v1) => {
                        v0 == v1
                },
                _ => false
                }
            },
            DataType::Bool(v0) => {
                match other{
                    DataType::Bool(v1) => {
                        v0 == v1
                },
                _ => false
                }   
            },
            DataType::Svg(v0) => {
                match other{
                    DataType::Svg(v1) => {
                        v0 == v1
                },
                _ => false
                }
            },
            DataType::Regex(v0) => {
                match other{
                    DataType::Regex(v1) => {
                        v0.as_str() == v1.as_str()
                },
                _ => false
                }
            },
        }
    }
    pub fn to_string(&self) -> String{
        match self{
            DataType::Char(c) => {c.clone()},
            _ => {panic!("char型ではありません")}
        }
    }
    pub fn to_i32(&self) -> i32{
        match self{
            DataType::Int(i) => {i.clone()},
            _ => {panic!("int型ではありません")}
        }
    }
    pub fn to_f64(&self) -> f64{
        match self{
            DataType::Float(f) => {f.clone()},
            _ => {panic!("floatt型ではありません")}
        }
    }
    pub fn to_bool(&self) -> bool{
        match self{
            DataType::Bool(b) => {b.clone()},
            _ => {panic!("bool型ではありません")}
        }
    }
    pub fn to_regex(&self) -> Regex{
        match self{
            DataType::Regex(r) => {r.clone()},
            _ => {panic!("Regex型ではありません")}
        }
    }
    pub fn to_svg(&self) -> SVG{
        match self{
            DataType::Svg(r) => {r.clone()},
            _ => {panic!("SVG型ではありません")}
        }
    }
}

#[derive(Debug)]
pub enum SubType{
    None,
    Id,
    NoneNegative,
    NaturalNumber,
    AligX,
    AligY,
    NameL,
    Name,
    Language,
    Gpos,
    Parsed,
    Aligment,
}

#[derive(Debug)]
pub struct Column{
    name:String,
    datatype:DataType,
    subtype:SubType,
    desc:String,
    order:i32
}

impl Column{

    pub fn new(name:&str,datatype:&DataType) -> Column {
        Column{
            name:name.to_string(),
            datatype:datatype.clone(),
            subtype:SubType::None,
            desc:"".to_string(),
            order:0,
        }
    }
    fn load(&mut self,val:serde_json::Value){
        self.datatype = match val["type"].as_str().unwrap(){
                "INTEGER" => DataType::Int(0),
                "float" => DataType::Float(0.0),
                "bool" => DataType::Bool(true),
                "char" => {
                    if val["name"].as_str().unwrap() == "Parsed" {DataType::Regex(Regex::new("").unwrap())}
                    else if val["name"].as_str().unwrap() == "Path" {DataType::Svg(SVG::new("M0,0"))}
                    else { DataType::Char("".to_string())}
            },
                &_ => DataType::Int(0),
            };
            self.subtype=SubType::None;
            //self.desc=val["desc"].as_str().unwrap().to_string();
            self.order=val["order"].as_i64().unwrap() as i32;
    }
    pub fn rename(&mut self,name:&str){
        self.name = name.to_string();
    }
    pub fn get_name<'a>(&'a mut self) -> &'a String{
        &self.name
    }
    pub fn get_datatype<'a>(&'a mut self) -> &'a DataType{
        &self.datatype
    }
}

#[derive(Debug)]
pub struct DataTable{
    pub name:String,
    column_list:Vec::<Column>,
    data_list:Vec::<Data>,
    desc:String,
}

impl DataTable{
    pub fn new(name:&str) -> DataTable {
        DataTable{
            name:name.to_string(),
            column_list:Vec::new(),
            data_list:Vec::new(),
            desc:"".to_string(),
        }
    }
    //Font内部の構造体にのみ作用するようにする

    pub fn rename(&mut self,name:&str){
        self.name = name.to_string();
    }
    pub fn get_name<'a>(&'a mut self) -> &'a String{
        &self.name
    }

    pub fn load(&mut self,val:serde_json::Value){
        let l = val["columnList"].as_object().unwrap();
        for (key,value) in l.iter() {
            if !value["dev"].as_bool().unwrap(){
                let k = self.create_column(key,&DataType::Int(0)).unwrap();
                k.load(value.clone());
            }
        }
        let l = val["data"].as_object().unwrap();
        for (_key,value) in l.iter() {
            let mut data : HashMap<String,DataType> = HashMap::new();
                let l_ = value.as_object().unwrap(); 
                for (key,value) in l_.iter() {
                let p = self.get_column(key);
                if p.is_some(){
                    match p.unwrap().get_datatype(){
                        DataType::Bool(_) => {data.insert(key.to_string(),DataType::Bool(value.as_bool().unwrap()));},
                        DataType::Char(_) => {
                            if key == "NameAlt" {
                                data.insert(key.to_string(),DataType::Char( Regex::new(r"\$(\d)").unwrap().replace_all(value.as_str().unwrap(),"$${$1}").to_string() ));
                                }
                            else {data.insert(key.to_string(),DataType::Char(value.as_str().unwrap().to_string()));
                            }
                        },
                        DataType::Svg(_) => {
                            data.insert(key.to_string(),DataType::Svg(SVG::new(value.as_str().unwrap())));

                        },
                        DataType::Regex(_) => {
                            data.insert(key.to_string(),DataType::Regex(Regex::new(value.as_str().unwrap()).unwrap()));
                        },
                        DataType::Float(_) => {data.insert(key.to_string(),DataType::Float(value.as_f64().unwrap()));},
                        DataType::Int(_) => {data.insert(key.to_string(),DataType::Int(value.as_i64().unwrap() as i32));},
                        }
                    }
                }
            let _ = &self.data_list.push(Data::new(&data));

        }
    }

    pub fn get_datalist<'a>(&'a mut self) -> Vec<&'a Data>{
        let mut ans: Vec::<&Data> = Vec::new();
        let mut i_len = 0;
        for _i in &self.data_list{
            i_len += 1;
            let _ = ans.push(&self.data_list[i_len-1]);
        }
        ans
    }

    pub fn get_single_data<'a>(&'a mut self) -> &'a Data{
        let ans = self.get_datalist();
        if ans.len() != 0{
            ans[0]
        } else {panic!("対象が存在しません")}
    }

    pub fn get_data_from_name<'a>(&'a mut self,param:&str) -> Result<Vec<&'a Data>,DataError>{
        let mut map = HashMap::new();
        map.insert(String::from("Name"), DataType::Char(param.to_string()));
        self.get_data(&map)
    }

    pub fn get_data_from_name_unwraped<'a>(&'a mut self,param:&str) -> Vec<&'a Data>{
        match self.get_data_from_name(param) {
            Ok(f) => f,
            Err(err) => panic!("{}",err.msg),
        }
    }

    pub fn get_single_data_from_name<'a>(&'a mut self,param:&str) -> Result<&'a Data,DataError>{
        match self.get_data_from_name(param){
            Ok(ans) =>{
            if ans.len() != 0{
                Ok(ans[0])
            } else {Err(DataError::new("対象が複数存在します"))}},
            Err(err) => Err(err),
        }
    }
    pub fn get_single_data_from_name_unwraped<'a>(&'a mut self,param:&str) -> &'a Data{
        match self.get_single_data_from_name(param) {
            Ok(f) => f,
            Err(err) => panic!("{}",err.msg),
        }
    }
    //unsafe多め
    pub fn get_data<'a>(&'a mut self,param:&HashMap<String,DataType>) -> Result<Vec<&'a Data>,DataError>{
        let mut ans: Vec::<&Data> = Vec::new();
        /*for (col,val) in param{
            let deletion = self.column_list.iter().position(|i| &i.name == col);
            if deletion.is_none(){
                ;
            }
        }*/
        let mut i_len = 0;
        'skip:for i in &self.data_list{
            i_len += 1;
            for (col,val0) in param{
                if let Some(val1) = &i.value_list.get(col){
                    match val0{
                        DataType::Regex(str0) =>{
                            match val1{
                                DataType::Char(str1) =>{
                                    if str0.captures(str1).is_none(){continue 'skip;}
                                },
                            _ => {continue 'skip;},
                            }
                        },
                        _ =>{
                            match val1{
                            DataType::Regex(_) =>{continue 'skip;},
                            _ => {
                            if !val0.is_equal(val1){continue 'skip;}
                                },
                            }
                        },
                    } 
                }   else {return Err(DataError::new("対象が存在しません"))}
            }

        let _ = ans.push(&self.data_list[i_len-1]);
        }
        Ok(ans)
    }

    pub fn get_data_unwraped<'a>(&'a mut self,param:&HashMap<String,DataType>) -> Vec<&'a Data>{
        match self.get_data(param){
            Ok(f) => f,
        Err(err) => panic!("{} は不正な値です",err.msg),
        }
    }
    pub fn get_data_single<'a>(&'a mut self,param:&HashMap<String,DataType>) -> Result<&'a Data,DataError>{
        let f = self.get_data(param);
        match f {
            Ok(data) =>  if data.len() == 1 {Ok(data[0])
            } else { Err(DataError::new("一致する値が複数あります"))},
            Err(err) => return Err(err),
        }
    }
    pub fn get_data_single_unwraped<'a>(&'a mut self,param:&HashMap<String,DataType>) -> &'a Data{
        match self.get_data_single(param) {
            Ok(f) => f,
            Err(err) => panic!("{}",err.msg),
        }
    }
    pub fn create_data<'a>(&'a mut self,param:&HashMap<String,DataType>) -> Result<Vec<&'a Data>,DataError>{
        if self.get_data(param).unwrap().len() >= 1{
            return Err(DataError::new("既に該当するDataが存在します"))
        } else {
            let creation = Data::new(param);
            let _ = &self.data_list.push(creation);
            return Ok(self.get_data(param).unwrap());
        }
    }
/*
    pub fn rename_data<'a>(&'a mut self,param:&HashMap<String,DataType>,param_alt:&HashMap<String,DataType>) -> Result<&'a mut Data,DataError>{
        {//二重借用の回避用
            if self.get_data(param_alt).is_some(){
                return Err(DataError::new("名称が重複します"))
            }
        }
        let subject = self.get_data(param);
        if subject.is_none(){
            return Err(DataError::new("対象が存在しません"));
        }
         else {
            let subject = subject.unwrap();
            //編集内容はここ
            return Ok(subject);
        }
    }

    pub unsafe fn delete_datatable(&mut self,param:&HashMap<String,DataType>) -> Result<bool,DataError>{
        let deletion = self.data_list.iter().position(|i| &i.valueList == param);
        if deletion.is_none(){
            return Err(DataError::new("対象が存在しません"));
        }
        let n = deletion.unwrap();
        let _ = self.data_list.remove(n);
        return Ok(true);//何返せばいいんだ
    }
*/
        //unsafe多め


        pub fn get_column<'a>(&'a mut self,name:&str) -> Option<&'a mut Column>{
            for i in &mut self.column_list{
                if i.name == name {return Some(i);}
            }
            return None;
        }
    
        pub fn create_column<'a>(&'a mut self,name:&str,dt:&DataType) -> Result<&'a mut Column,DataError>{
            if self.get_column(name).is_some(){
                return Err(DataError::new("既にそのFontが存在します"))
            } else {
                let creation = Column::new(name,dt);
                let _ = &self.column_list.push(creation);
                return Ok(self.get_column(name).unwrap());
            }
        }

}

#[derive(Debug)]
pub struct DataError {
    /// エラーメッセージ
    pub msg: String,
}

impl DataError {
    fn new(msg: &str) -> DataError {
        DataError {
            msg: msg.to_string(),
        }
    }
}


#[derive(Debug)]
pub struct Data{
    value_list:HashMap<String,DataType>
}

impl Data{
    fn new(v:&HashMap<String,DataType>) -> Data {
        Data{
            value_list:v.clone(),
        }
    }
    pub fn get_value<'a>(&'a self,s:&'a str) -> &'a DataType {
        self.value_list.get(s).unwrap()
    }
    pub fn get_value_as_i32<'a>(&'a self,s:&'a str) -> i32 {
        self.get_value(s).to_i32()
    }
    pub fn get_value_as_f64<'a>(&'a self,s:&'a str) -> f64 {
        self.get_value(s).to_f64()
    }
    pub fn get_value_as_bool<'a>(&'a self,s:&'a str) -> bool {
        self.get_value(s).to_bool()
    }
    pub fn get_value_as_string<'a>(&'a self,s:&'a str) -> String {
        self.get_value(s).to_string()
    }
    pub fn get_value_as_regex<'a>(&'a self,s:&'a str) -> Regex {
        self.get_value(s).to_regex()
    }
    pub fn get_value_as_svg<'a>(&'a self,s:&'a str) -> SVG {
        self.get_value(s).to_svg()
    }
}