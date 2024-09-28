mod thaqqa;

use thaqqa::glyph::{Glyphvec,Aligment};
use thaqqa::database::fonttable::FontTable;

use thaqqa::parser::{Parser,Language};
use thaqqa::gsub::Gsub;

use thaqqa::svg::svg::{Point,SVG};

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use std::{fs::File,io::BufReader};
use std::time;

fn calc_lag(time : i32,letter : &str,language : &Language,aligment : &Aligment, font : &str){
    let mut heikin = std::time::Duration::new(0,0);
    for i in 0..time{
        let now = time::Instant::now(); //10回計測
        //suzusha koratl\nnaylath kaatak\nnaylath nghadhanel\nkako karuta

        //wiqhenvwe

        //wiqhenvwe -> 284.13399ms
        //athakhikaki -> 313.30039ms
        //athaghikaki -> 317.05993ms ←？？？？？？？？？
        //suzusha koratl\nnaylath kaatak\nnaylath nghadhanel\nkako karuta
        //-> 1.40956126s

        let k = Parser::parse(letter,language);
        let k = Gsub::gsub(&k,aligment,font);
        let _l = Glyphvec::new(16.0,&k,aligment,font,"","");

        let jkn = now.elapsed();
        heikin += jkn;
        println!("{}回目:{:?}",i,jkn);
        
    }
    println!("平均時間：{:?}", heikin/10);
}

fn encode(letter : &str,language : &Language,aligment : &Aligment, font : &str){
    let d = calc(letter,language,aligment,font);
    let brdr = d.get_border();
    let mut data = Data::new();
    for p in d.point_array {
        match p{
        Point::M(l,n0,n1) => {
            data = if l {data.move_to((n0,n1))} else {data.move_by((n0,n1))};
        },
        Point::H(l,n0) => {
            data = if l {data.horizontal_line_to(n0)} else {data.horizontal_line_by((n0))};
        },
        Point::V(l,n0) => {
            data = if l {data.vertical_line_to(n0)} else {data.vertical_line_by((n0))};
        },
        Point::L( l, n0, n1) => {
            data = if l {data.line_to((n0,n1))} else {data.line_by((n0,n1))};
        },
        Point::T( l, n0, n1) => {
            data = if l {data.smooth_quadratic_curve_to((n0,n1))} else {data.smooth_quadratic_curve_by((n0,n1))};
        },
        Point::Q( l, n0, n1, n2, n3) => {
            data = if l {data.quadratic_curve_to((n0,n1,n2,n3))} else {data.quadratic_curve_by((n0,n1,n2,n3))};
        },
        Point::S( l, n0, n1, n2, n3)  => {
            data = if l {data.smooth_cubic_curve_to((n0,n1,n2,n3))} else {data.smooth_cubic_curve_by((n0,n1,n2,n3))};
        },
        Point::C( l, n0, n1, n2, n3,n4,n5)  => {
            data = if l {data.cubic_curve_to((n0,n1,n2,n3,n4,n5))} else {data.cubic_curve_by((n0,n1,n2,n3,n4,n5))};
        },
        Point::A( l, n0, n1, n2, n3,n4,n5,n6)  => {
            data = if l {data.elliptical_arc_to((n0,n1,n2,n3,n4,n5,n6))} else {data.elliptical_arc_by((n0,n1,n2,n3,n4,n5,n6))};
        },
        Point::Z(_)  => {data = data.close();},
        }
    }
    let path = Path::new()
    .set("fill", "black")
    .set("stroke", "none")
    .set("stroke-width", 0)
    .set("d", data.clone());

let document = Document::new()
    .set("viewBox", (brdr.0, brdr.1, brdr.2-brdr.0, brdr.3-brdr.1))
    .add(path);

    let vector = match aligment{
        &Aligment::Vertical => "v_",
        &Aligment::Right => "r_",
        &Aligment::Left => "l_",
    };
svg::save(format!("{}{}.svg",vector,URL_SAFE.encode(letter)), &document).unwrap();
}

fn load_root(){
    let file = File::open("./src/fontjson/root.json").unwrap();
    let reader = BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(reader).unwrap();
    FontTable::load(json);
}

fn load_font(font : &str){
    let file = File::open(format!("./src/fontjson/font/{}.json",font)).unwrap();
    let reader = BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(reader).unwrap();
    FontTable::load_font(json,font);
}

fn calc(letter : &str,language : &Language,aligment : &Aligment, font : &str) -> SVG{
    //Parser：文字列を東果/汽陸語用の内部表記に変換します
    //GSUB：実際の表記を設定します
    //Glyphvec：結合させて表示させます
    let k = Parser::parse(letter,language);
    let k = Gsub::gsub(&k,aligment,font);
    let l = Glyphvec::new(16.0,&k,aligment,font,"","");
    l.d
}
fn main(){
    load_root();
    load_font("default");
    
    let l = encode(&"khehyelughunuy,\nnehthvnu.",&Language::Khehyelu,&Aligment::Vertical,&"default");
    //FontTable::drop();//メモリ開放用
}