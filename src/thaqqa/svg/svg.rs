extern crate regex;
use regex::Regex;
//use std::fmt;   // フォーマットの標準ライブラリクレート
use std::f64::{INFINITY, NEG_INFINITY};

use super::super::database::fonttable::FontTable;

#[derive(Debug,Clone,PartialEq)]
pub enum Point {
    V(bool,f64),H(bool,f64),
    L(bool,f64,f64),M(bool,f64,f64),
    S(bool,f64,f64,f64,f64),C(bool,f64,f64,f64,f64,f64,f64),
    T(bool,f64,f64),Q(bool,f64,f64,f64,f64),
    A(bool,f64,f64,f64,f64,f64,f64,f64),Z(bool),
}

impl Point{
    fn to_abs() {
    }
    fn get_type(&self) -> char {
        let f = |text:char,l:bool| -> char {
            if l {text.to_ascii_uppercase()}
            else {text.to_ascii_lowercase()}
        };
        match self {
            Point::Z(l) => f('Z',*l),
            Point::M(l,_,_) => f('M',*l),
            Point::L(l,_,_) => f('L',*l),
            Point::T(l,_,_) => f('T',*l),
            Point::H(l,_) => f('H',*l),
            Point::V(l,_) => f('V',*l),
            Point::Q(l,_,_,_,_) => f('Q',*l),
            Point::S(l,_,_,_,_) => f('S',*l),
            Point::C(l,_,_,_,_,_,_) => f('C',*l),
            Point::A(l,_,_,_,_,_,_,_) => f('A',*l),
        }
    }
    
    fn to_string(&self) -> String {
        let f = |text:char,l:bool| -> String {
            if l {text.to_ascii_uppercase().to_string()}
            else {text.to_ascii_lowercase().to_string()}
        };
        match self {
            Point::Z(l) => f('Z',*l),
            Point::M(l,n0,n1) => format!("{}{},{}",f('M',*l),n0,n1),
            Point::L(l,n0,n1) => format!("{}{},{}",f('L',*l),n0,n1),
            Point::T(l,n0,n1) => format!("{}{},{}",f('T',*l),n0,n1),
            Point::H(l,n0) => format!("{}{}",f('H',*l),n0),
            Point::V(l,n0) => format!("{}{}",f('V',*l),n0),
            Point::Q(l,n0,n1,n2,n3) => format!("{}{},{} {},{}",f('Q',*l),n0,n1,n2,n3),
            Point::S(l,n0,n1,n2,n3) => format!("{}{},{} {},{}",f('S',*l),n0,n1,n2,n3),
            Point::C(l,n0,n1,n2,n3,n4,n5) => format!("{}{},{} {},{} {},{}",f('C',*l),n0,n1,n2,n3,n4,n5),
            Point::A(l,n0,n1,n2,n3,n4,n5,n6) => format!("{}{},{},{},{},{},{},{}",f('A',*l),n0,n1,n2,n3,n4,n5,n6),
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct SVG{
    pub point_array:Vec<Point>
}

impl SVG{
    pub fn new(str : &str) -> SVG{ //Stringから作る
        let regex_datatable =  FontTable::get_datatable_unwraped("REGEX","");

        
        let mut ans : Vec<Point> = Vec::new();
        let seperator = regex_datatable.get_single_data_from_name_unwraped("Sep_SVG").get_value_as_regex("Parsed");
        let mut a = seperator.split(str);
        let re_ = regex_datatable.get_single_data_from_name_unwraped("extract_name").get_value_as_regex("Parsed");
        loop{
            let a_ = match a.next(){
                None | Some("") => break,
                Some(i) => i,
            };
            let re = match re_.captures(a_){
                None => 0,
                Some(i) => i[0].len(),
            };
            let a0 = &a_[..re];
            if a0.starts_with("Z"){ans.push(Point::Z(true));}
            if a0.starts_with("z"){ans.push(Point::Z(false));}

            fn nx(itr : Option<&str>) -> f64 {itr.unwrap().parse::<f64>().unwrap()}

            let a1 = match &a_[re..].parse::<f64>(){
                Ok(r) => *r,
                Err(_e) => if !a0.ends_with("Z") && !a0.ends_with("z") {nx(a.next())}
                else {0.0},
            };
            let last = match a0.chars().last() {
                Some(_) =>  a0.chars().last().unwrap(),
                None => ans.last().unwrap().get_type()};
                
                match last.to_ascii_uppercase(){
                    'Z' => (),
                    'M' => ans.push(Point::M(last.is_uppercase(),a1,nx(a.next()) )),
                    'L' => ans.push(Point::L(last.is_uppercase(),a1,nx(a.next()) )),
                    'T' => ans.push(Point::T(last.is_uppercase(),a1,nx(a.next()) )),
                    'H' => {ans.push(Point::H(last.is_uppercase(),a1));},
                    'V' => {ans.push(Point::V(last.is_uppercase(),a1));},
                    'Q' => ans.push(Point::Q(last.is_uppercase(),a1,nx(a.next()),nx(a.next()),nx(a.next()) )),
                    'S' => ans.push(Point::S(last.is_uppercase(),a1,nx(a.next()),nx(a.next()),nx(a.next()) )),
                    'C' => ans.push(Point::C(last.is_uppercase(),a1,nx(a.next()),nx(a.next()),nx(a.next()),nx(a.next()),nx(a.next()) )),
                    'A' => ans.push(Point::A(last.is_uppercase(),a1,nx(a.next()),nx(a.next()),nx(a.next()),nx(a.next()),nx(a.next()),nx(a.next()) )),
                    _ => panic!()
                }
                
        }
        SVG {
            point_array : ans
        }
    }
//fn convert(&mut self,bool isupper){}
    pub fn to_rel(&mut self){
        let mut abspos =(0.0,0.0);
        let mut startpos = (0.0,0.0);
        for a in &mut self.point_array{
            match a {
                Point::M(ref mut l,ref mut n0,ref mut n1) => {
                    if *l {let temp = (*n0,*n1);
                        *n0 -= abspos.0;*n1 -= abspos.1;
                        abspos = temp;
                        startpos = abspos;
                        *l = false;
                    } else {
                    abspos.0 += *n0;abspos.1 += *n1;
                    startpos = abspos;
                    }
                },
                Point::H(ref mut l,ref mut n0) => {
                    if *l {let temp = *n0;
                        *n0 -= abspos.0;abspos.0 = temp;
                        *l = false;
                    } else {
                    abspos.0 += *n0;
                    }
                },
                Point::V(ref mut l,ref mut n1) => {
                    if *l {let temp = *n1;
                        *n1 -= abspos.1;abspos.1 = temp;
                        *l = false;
                    } else {
                    abspos.1 += *n1;
                    }
                },
                Point::L(ref mut l,ref mut n0,ref mut n1) | Point::T(ref mut l,ref mut n0,ref mut n1) => {
                    if *l {let temp = (*n0,*n1);
                        *n0 -= abspos.0;*n1 -= abspos.1;
                        abspos = temp;
                        *l = false;
                    } else {
                    abspos.0 += *n0;abspos.1 += *n1;
                    }
                },
                Point::S(ref mut l,ref mut n0,ref mut n1,ref mut n2,ref mut n3) | Point::Q(ref mut l,ref mut n0,ref mut n1,ref mut n2,ref mut n3) => {
                    if *l {let temp = (*n2,*n3);
                        *n0 -= abspos.0;*n1 -= abspos.1;*n2 -= abspos.0;*n3 -= abspos.1;
                        abspos = temp;
                        *l = false;
                    } else {
                        abspos.0 += *n2; abspos.1 += *n3;
                    }
                },
                Point::C(ref mut l,ref mut n0,ref mut n1,ref mut n2,ref mut n3,ref mut n4,ref mut n5)  => {
                    if *l {let temp = (*n4,*n5);
                        *n0 -= abspos.0;*n1 -= abspos.1;*n2 -= abspos.0;*n3 -= abspos.1;*n4 -= abspos.0;*n5 -= abspos.1;
                        abspos = temp;
                        *l = false;
                    } else {
                        abspos.0 += *n4; abspos.1 += *n5;
                    }
                },
                Point::A(ref mut l,_,_,_,_,_,ref mut n5,ref mut n6)  => {
                    if *l {let temp = (*n5,*n6);
                        *n5 -= abspos.0;*n6 -= abspos.1;
                        abspos = temp;
                        *l = false;
                    } else {
                        abspos.0 += *n5; abspos.1 += *n6;
                    }
                },
                Point::Z(ref mut l)  => {
                    abspos.0 = startpos.0;abspos.1 = startpos.1;
                    startpos.0 = 0.0;startpos.1 = 0.0;
                    *l = false;
                },
            }
            
        }
    }

    pub fn to_abs(&mut self){
        let mut abspos =(0.0,0.0);
        let mut startpos = (0.0,0.0);
        for a in &mut self.point_array{
            match a {
                Point::M(ref mut l,ref mut n0,ref mut n1) => {
                    if *l {
                        abspos.0 = *n0;abspos.1 = *n1;
                        startpos = abspos;
                    }else{
                        let temp = (*n0,*n1);
                        *n0 += abspos.0;*n1 += abspos.1;
                        abspos.0 += temp.0;abspos.1 += temp.1;
                        startpos = abspos;
                        *l = true;
                    }
                },
                Point::H(ref mut l,ref mut n0) => {
                    if *l {abspos.1 = *n0;
                    }else{
                        let temp = *n0;
                        *n0 += abspos.0;abspos.0 += temp;
                        *l = true;
                    }
                },
                Point::V(ref mut l,ref mut n1) => {
                    if *l {abspos.1 = *n1;
                    }else{
                        let temp = *n1;
                        *n1 += abspos.1;abspos.1 += temp;
                        *l = true;
                    }
                },
                Point::L(ref mut l,ref mut n0,ref mut n1) | Point::T(ref mut l,ref mut n0,ref mut n1) => {
                    if *l {abspos.0 = *n0;abspos.1 = *n1;
                    }else{
                        let temp = (*n0,*n1);
                        *n0 += abspos.0;*n1 += abspos.1;
                        abspos.0 += temp.0;abspos.1 += temp.1;
                        *l = true;
                    }
                },
                Point::S(ref mut l,ref mut n0,ref mut n1,ref mut n2,ref mut n3) | Point::Q(ref mut l,ref mut n0,ref mut n1,ref mut n2,ref mut n3) => {
                    if *l {
                        abspos.0 = *n2; abspos.1 = *n3;
                    }else{
                        let temp = (*n2,*n3);
                        *n0 += abspos.0;*n1 += abspos.1;*n2 += abspos.0;*n3 += abspos.1;
                        abspos.0 += temp.0;abspos.1 += temp.1;
                        *l = true;
                    }
                },
                Point::C(ref mut l,ref mut n0,ref mut n1,ref mut n2,ref mut n3,ref mut n4,ref mut n5)  => {
                    if *l {
                        abspos.0 = *n4; abspos.1 = *n5;
                    }else{
                        let temp = (*n4,*n5);
                        *n0 += abspos.0;*n1 += abspos.1;*n2 += abspos.0;*n3 += abspos.1;*n4 += abspos.0;*n5 += abspos.1;
                        abspos.0 += temp.0;abspos.1 += temp.1;
                        *l = true;
                    }
                },
                Point::A(ref mut l,_,_,_,_,_,ref mut n5,ref mut n6)  => {
                    if *l {
                        abspos.0 = *n5; abspos.1 = *n6;
                    }else{
                        let temp = (*n5,*n6);
                        *n5 += abspos.0;*n6 += abspos.1;
                        abspos.0 += temp.0;abspos.1 += temp.1;
                        *l = true;
                    }
                },
                Point::Z(ref mut l)  => {
                    abspos.0 = startpos.0;abspos.1 = startpos.1;
                    startpos.0 = 0.0;startpos.1 = 0.0;
                    *l = false;
                },
            }
        }
    }
    pub fn get_left(&self)->f64{
        self.get_border().0
    }
    pub fn get_top(&self)->f64{
        self.get_border().1
    }
    pub fn get_right(&self)->f64{
        self.get_border().2
    }
    pub fn get_bottom(&self)->f64{
        self.get_border().3
    }
    pub fn get_border(&self) -> (f64,f64,f64,f64){
        let mut abspos =(0.0,0.0);
        let mut startpos = (0.0,0.0);
        let mut atai = (INFINITY,INFINITY,NEG_INFINITY,NEG_INFINITY);
        for a in &self.point_array{
            match a {
                Point::M(ref l,ref n0,ref n1) => {
                    if *l {
                        abspos.0 = *n0;abspos.1 = *n1;
                    } else {
                    abspos.0 += *n0;abspos.1 += *n1;
                    startpos = abspos;
                    }
                },
                Point::H(ref l,ref n0) => {
                    if *l {
                        abspos.0 = *n0;
                    } else {
                    abspos.0 += *n0;
                    }
                },
                Point::V(ref l,ref n1) => {
                    if *l {
                        abspos.1 = *n1;
                    } else {
                    abspos.1 += *n1;
                    }
                },
                Point::L(ref l,ref n0,ref n1) | Point::T(ref l,ref n0,ref n1) => {
                    if *l {
                        abspos.0 = *n0;abspos.1 = *n1;
                    } else {
                    abspos.0 += *n0;abspos.1 += *n1;
                    }
                },
                Point::S(ref l,_,_,ref n2,ref n3) | Point::Q(ref l,_,_,ref n2,ref n3) => {
                    if *l {
                        abspos.0 = *n2;abspos.1 = *n3;
                    } else {
                    abspos.0 += *n2;abspos.1 += *n3;
                    }
                },
                Point::C(ref l,_,_,_,_,ref n4,ref n5)  => {
                    if *l {
                        abspos.0 = *n4;abspos.1 = *n5;
                    } else {
                    abspos.0 += *n4;abspos.1 += *n5;
                    }
                },
                Point::A(ref l,_,_,_,_,_,ref n5,ref n6)  => {
                    if *l {
                        abspos.0 = *n5;abspos.1 = *n6;
                    } else {
                    abspos.0 += *n5;abspos.1 += *n6;
                    }
                },
                Point::Z(_)  => {
                    abspos.0 = startpos.0;abspos.1 = startpos.1;
                    startpos.0 = 0.0;startpos.1 = 0.0;
                },
            }
            atai.0 = f64::min(atai.0,abspos.0);
            atai.1 = f64::min(atai.1,abspos.1);
            atai.2 = f64::max(atai.2,abspos.0);
            atai.3 = f64::max(atai.3,abspos.1);
        }
        if atai.0 == INFINITY {
            atai.0 = 0.0;
        }
        if atai.1 == INFINITY {
            atai.1 = 0.0;
        }
        if atai.2 == NEG_INFINITY {
            atai.2 = 0.0;
        }
        if atai.3 == NEG_INFINITY {
            atai.3 = 0.0;
        }
        atai
    }
    pub fn merge_two(&mut self, x0 : f64 , y0 : f64,other0 : SVG){
        let _ = &self.point_array.extend(vec![Point::M(true,x0,y0)]);
        let _ = &self.point_array.extend(other0.point_array);
    }
    pub fn merge_three(&mut self, x0 : f64 , y0 : f64,other0 : SVG , x1 : f64 , y1 : f64,other1 : SVG){
        let _ = &self.point_array.extend(vec![Point::M(true,x0,y0)]);
        let _ = &self.point_array.extend(other0.point_array);
        let _ = &self.point_array.extend(vec![Point::M(true,x1,y1)]);
        let _ = &self.point_array.extend(other1.point_array);
    }
    pub fn set_gap_alter(&mut self, x0 : f64 , y0 : f64){
        let _ = &self.point_array.extend(vec![Point::M(true,x0,y0)]);
    }
    pub fn set_gap(&mut self, x : f64 , y : f64){
        let _ = &self.point_array.insert(0,Point::M(true,x,y));
    }
    pub fn build_m(x0 : f64 , y0 : f64) -> SVG{
        SVG { point_array : vec![Point::M(true,x0,y0)]}
    }
    pub fn build_skewer(x0 : f64 , y0 : f64 , x1 : f64 , y1 : f64) -> SVG{
        if x1<x0 || y1<y0 { SVG { point_array : Vec::new()}}
        else { SVG { point_array : vec![Point::M(true,x0,y0),Point::V(true,y1),Point::H(true,x1),Point::V(true,y0),Point::Z(true),Point::M(true,0.0,y0)]}}
    }
    pub fn to_string(&self) -> String{
        let i: Vec<String> = self.point_array.iter().map(|x| x.to_string()).collect();
        i.join(" ")
    }
    pub fn rescale(&mut self, size : f64){
        self.to_abs();
        for a in &mut self.point_array{
            match a {
                Point::M(_,ref mut n0,ref mut n1) => {
                    *n0 *= size;*n1 *= size;
                },
                Point::H(_,ref mut n0) => {
                    *n0 *= size;
                },
                Point::V(_,ref mut n1) => {
                    *n1 *= size;
                },
                Point::L(_,ref mut n0,ref mut n1) | Point::T(_,ref mut n0,ref mut n1) => {
                    *n0 *= size;*n1 *= size;
                },
                Point::S(_,ref mut n0,ref mut n1,ref mut n2,ref mut n3) | Point::Q(_,ref mut n0,ref mut n1,ref mut n2,ref mut n3) => {
                    *n0 *= size;*n1 *= size;*n2 *= size;*n3 *= size;
                },
                Point::C(_,ref mut n0,ref mut n1,ref mut n2,ref mut n3,ref mut n4,ref mut n5)  => {
                    *n0 *= size;*n1 *= size;*n2 *= size;*n3 *= size;*n4 *= size;*n5 *= size;
                },
                Point::A(_,ref mut n0,ref mut n1,ref mut n2,ref mut n3,ref mut n4,ref mut n5,ref mut n6)  => {
                    *n0 *= size;*n1 *= size;*n2 *= size;*n3 *= size;*n4 *= size;*n5 *= size;*n6 *= size;
                },
                Point::Z(_)  => {
                },
            }
        }
        self.to_rel();
    }
    pub fn len(&self) -> usize{
        self.point_array.len()
    }
}