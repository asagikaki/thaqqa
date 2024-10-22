extern crate may_minihttp;

mod thaqqa;
use thaqqa::glyph::{Glyphvec,Aligment};
use thaqqa::database::fonttable::FontTable;

use thaqqa::parser::{Parser,Language};
use thaqqa::gsub::Gsub;

use thaqqa::svg::svg::{Point,SVG};

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

use std::io;
use may_minihttp::{HttpServer, HttpService, Request, Response};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use std::{fs::File,io::BufReader};

#[derive(Clone)]
struct HelloWorld;


fn encode(letter : &str,language : &Language,aligment : &Aligment, font : &str) {
    let d = calc(letter,language,aligment,font).unwrap();
    let brdr = d.get_border();
    let mut data = Data::new();
    for p in d.point_array {
        match p{
        Point::M(l,n0,n1) => {
            if l {data = data.move_to((n0,n1));} else {data = data.move_by((n0,n1));}
        },
        Point::H(l,n0) => {
            if l {data = data.horizontal_line_to(n0);} else {data = data.horizontal_line_by((n0));}
        },
        Point::V(l,n0) => {
            if l {data = data.vertical_line_to(n0);} else {data = data.vertical_line_by((n0));}
        },
        Point::L( l, n0, n1) => {
            if l {data = data.line_to((n0,n1));} else {data = data.line_by((n0,n1));}
        },
        Point::T( l, n0, n1) => {
            if l {data = data.smooth_quadratic_curve_to((n0,n1));} else {data = data.smooth_quadratic_curve_by((n0,n1));}
        },
        Point::Q( l, n0, n1, n2, n3) => {
            if l {data = data.quadratic_curve_to((n0,n1,n2,n3));} else {data = data.quadratic_curve_by((n0,n1,n2,n3));}
        },
        Point::S( l, n0, n1, n2, n3)  => {
            if l {data = data.smooth_cubic_curve_to((n0,n1,n2,n3));} else {data = data.smooth_cubic_curve_by((n0,n1,n2,n3));}
        },
        Point::C( l, n0, n1, n2, n3,n4,n5)  => {
            if l {data = data.cubic_curve_to((n0,n1,n2,n3,n4,n5));} else {data = data.cubic_curve_by((n0,n1,n2,n3,n4,n5));}
        },
        Point::A( l, n0, n1, n2, n3,n4,n5,n6)  => {
            if l {data = data.elliptical_arc_to((n0,n1,n2,n3,n4,n5,n6));} else {data = data.elliptical_arc_by((n0,n1,n2,n3,n4,n5,n6));}
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

fn calc(letter : &str,language : &Language,aligment : &Aligment, font : &str) -> Result<SVG,String>{
    //Parser：文字列を東果/汽陸語用の内部表記に変換します
    //GSUB：実際の表記を設定します
    //Glyphvec：内部で
    if FontTable::get_datatable("GPOS",font).is_none() { Err(String::from("font was not found"))}
    else{
        let k = Parser::parse(letter,language);
        let k = Gsub::gsub(&k,aligment,font);
        let l = Glyphvec::new(16.0,&k,aligment,font,"","");
        Ok(l.d)
    }
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

impl HttpService for HelloWorld {
    fn call(&mut self, req: Request, rsp: &mut Response) -> io::Result<()> {
        println!("req: {:?}",req.path());
        // /api/<fontname>/language/(vert|left|right)/<文字列>/?size=<size>&&...　といった形か
        // ↑はapi/<fontname>/(vert|left|right)というフォルダの中で
        //<文字列部分のBase64>&&クエリ部分のbase64&&...のようにしてファイルに保存する
        let (code , message, body_strinng) = match req.path() {
            //"/200" => (200, "OK"),
            //"/400" => (400, "Bad Request"),
            //"/500" => (500, "Internal Server Error"),
            p if p.starts_with("/api") => {
                rsp.header("Content-Type: application/json");
                let ans:Vec<&str> = p.split("/").collect();
                println!("{:?}",ans);
                if ans.len() < 6 || 8 <= ans.len() || (ans.len() == 7 && !ans[6].starts_with("?")) { (404, "Not Found",String::from("Not Found")) } // そんなもの作りようがあるか
                else {
                    let lang = match ans[3]{
                        "khhyl" => Some(&Language::Khehyelu),
                        "sangl" => Some(&Language::Sanghal),
                        "raw" => Some(&Language::Sanghal),
                        _ => None
                    };
                    let alig = match ans[4]{
                        "vert" => Some(&Aligment::Vertical),
                        "left" => Some(&Aligment::Left),
                        "right" => Some(&Aligment::Right),
                        _ => None
                    };
                    
                    if lang.is_none() || alig.is_none() {(404, "Not Found",String::from("Not Found"))}
                    else {
                        (200, "OK", match calc(ans[5],lang.unwrap(),alig.unwrap(),ans[2]){
                            Ok(svg) => svg.to_string(),
                            Err(str) => str,
                        })
                    }
                }
                
            },
            _ => {(404, "Not Found",String::from("Not Found"))},
        };
        rsp.status_code(code , message);
        rsp.body_vec(body_strinng.into_bytes());
        Ok(())
    }
}

// Start the server in `main`.
fn main() {
    load_root();
    load_font("default");

    let server = HttpServer(HelloWorld).start("0.0.0.0:8080").unwrap();
    server.join().unwrap();
}