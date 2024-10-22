
use super::svg::svg::SVG;
use super::database::fonttable::FontTable;
use super::database::datatable::DataType;
use std::collections::HashMap;

//use rayon::prelude::*;

use std::{thread, time};

#[derive(Clone,PartialEq,Debug)]
pub enum Aligment{
    Right,
    Left,
    Vertical,
}

enum Form{
    Onset,
    Coda,
    Sol,
    Init,
    Mid,
    Fin,
    None,
}

enum GlyphType{
    Syllable,
    Punctuation,
    Vowel,
    Consonant,
    P_Diacritical,
    V_Diacritical,
    C_Diacritical,
    None
}

pub struct Glyphvec{
    font:String,
    aligment:Aligment,
    name:String,
    pub d:SVG,
}

impl Glyphvec{
    pub fn new(size:f64,name:&str,aligment:&Aligment,font:&str,prev:&str,next:&str) -> Glyphvec{
        let d_ = Self::calc_vec(size,&name,&aligment,&font,&prev,&next);
        Glyphvec{
            font:font.to_string(),
            aligment:aligment.clone(),
            name:name.to_string(),
            d:d_,
        }
    }

    fn calc_vec(size:f64,name:&str,aligment:&Aligment,font:&str,prev:&str,next:&str) -> SVG{
        let default_x = 0.0;let default_y = 0.0; //あとで変更可能にする
        
        let fontdata = FontTable::get_datatable_unwraped("FONTDATA",font).get_single_data();

        let regex_datatable =  FontTable::get_datatable_unwraped("REGEX","");

        let regx_insert_sep = regex_datatable.get_single_data_from_name_unwraped("(L()L())").get_value_as_regex("Parsed");
        let regx_seperator = regex_datatable.get_single_data_from_name_unwraped("Seperate").get_value_as_regex("Parsed");
        let alt = "${1}Sep()${2}";
        let r_ = regx_insert_sep.replace_all(&name,alt);
        let r_ = regx_insert_sep.replace_all(&r_,alt);
        
        let mut d_list: Vec<_> = regx_seperator.split(&r_).collect();

        if aligment == &Aligment::Right {
            d_list.reverse();
        }
        
        let mut d_ = SVG::build_m(default_x,default_y);
        let mut x = default_x; let mut y = default_y;


        match aligment{
            Aligment::Vertical =>{
                let w = fontdata.get_value_as_f64("WidthV");
                let em_ = |n:f64| n/w;
                let mut isnewline = true;

                let p_detector = regex_datatable.get_single_data_from_name_unwraped("P(*)").get_value_as_regex("Parsed");
                let isol_detector = regex_datatable.get_single_data_from_name_unwraped("*(sol").get_value_as_regex("Parsed");
                let newline_detector = regex_datatable.get_single_data_from_name_unwraped("P(newline)").get_value_as_regex("Parsed");
                let mut isol_detector_result = vec![];

                for i in 0..d_list.len(){
                    
                    if i == 0 {isol_detector_result.push(isol_detector.captures(d_list[i]).is_some());}
                    if i != d_list.len()-1 {isol_detector_result.push(isol_detector.captures(d_list[i+1]).is_some());}

                    let (_,_a,_,_b) = Glyph::get_gpos(d_list[i],aligment,"INEX",font);

                    let str_prev = if i == 0 { if prev == "" {"Null()".to_string()} else {prev.to_string()}}else{d_list[i-1].to_string()} + d_list[i];
                    let str_next = d_list[i].to_string() + if i != d_list.len()-1 {d_list[i+1]} else {if next == "" {"Null()"} else {next}};
                    
                    let gap_above = _a + Glyph::get_gpos(&str_prev,aligment,"EX",font).3;
                    let gap_below = _b + Glyph::get_gpos(&str_next,aligment,"EX",font).1;

                    y += gap_above;
                        
                    let v_ = match p_detector.captures(d_list[i]){
                        None => "".to_string(),
                        Some(i) => i[0].to_string(),
                    };
                        
                    let d_new = Glyph::new(d_list[i],&aligment,&font,-gap_above,gap_below).d;

                    let (_,d_top,_,d_bottom) = d_new.get_border();
                    if isnewline{
                        isnewline = false;
                    } else if i != 0{
                        let cond_tmp = isol_detector_result[i-1] || !isol_detector_result[i];
                        y -= d_top - if cond_tmp {f64::min(0.0,-gap_above)} else {-gap_above};
                    }
                    d_.merge_two(x,y,d_new);
                    
                    let cond_tmp = isol_detector_result[i] ||  i == d_list.len()-1 || !isol_detector_result[i+1];
                    y += d_bottom + if cond_tmp {f64::min(0.0,gap_below)} else {gap_below};
                    
                    if newline_detector.captures(d_list[i]).is_some() {
                        isnewline = true;
                        x -= w; y = 0.0;
                    }
                }
                d_.set_gap_alter(x,y);
                d_.rescale(em_(size));
            },
            _ => {
                let weight = fontdata.get_value_as_f64("WidthV");
                let height = fontdata.get_value_as_f64("Height");
                let isrel = fontdata.get_value_as_bool("isWidthHRel");
                let topgap_h = fontdata.get_value_as_f64("TopGapH");

                let em_ = |n:f64| n/height;
                let isleft = aligment == &Aligment::Left;

                let p_detector = regex_datatable.get_single_data_from_name_unwraped("P(*)").get_value_as_regex("Parsed");
                let newline_detector = regex_datatable.get_single_data_from_name_unwraped("P(newline)").get_value_as_regex("Parsed");
                for i in 0..d_list.len(){
                    let (_l,_g,_r,_) = Glyph::get_gpos(d_list[i],aligment,"INEX",font);
 
                    let str_prev = if i == 0 { if prev == "" {"Null()".to_string()} else {prev.to_string()}}else{d_list[i-1].to_string()} + d_list[i];

                    let str_next = d_list[i].to_string() + if i != d_list.len()-1 {d_list[i+1]} else {if next == "" {"Null()"} else {next}};
                    
                    let d_new = Glyph::new(d_list[i],&aligment,&font,height,_g+height/2.0).d;
                    let gap_left = _l + Glyph::get_gpos(&str_prev,aligment,"EX",font).2;
                    let gap_right = _r + Glyph::get_gpos(&str_next,aligment,"EX",font).0;
    
                    let x_temp = if !isrel { weight } else {
                        let (d_left,_,d_right,_) = d_new.get_border();
                        -d_left+d_right+weight}/2.0;
                    x +=  x_temp + (if isleft {gap_left} else {gap_right}) ;
                    d_.merge_two(x,y,d_new);

                    if i != d_list.len()-1 { x += x_temp + (if !isleft {gap_left} else {gap_right});}

                    let v_ = match p_detector.captures(d_list[i]){
                        None => "".to_string(),
                        Some(i) => i[0].to_string(),
                    };
                    if newline_detector.captures(d_list[i]).is_some(){
                        x = 0.0; y += topgap_h+height;
                    }
                }
                d_.set_gap_alter(x,y);
                d_.rescale(em_(size));
            }
        }

        d_
    }
}

pub struct Glyph{
    font:String,
    aligment:Aligment,
    name:String,
    pub d:SVG,
    //attribute:,
}

impl Glyph{
    pub fn new(name:&str,aligment:&Aligment,font:&str,val_above:f64,val_below:f64) -> Glyph{
        
        let d_ = Self::calc_d(name,&aligment,font,val_above,val_below);
        Glyph{
            font:font.to_string(),
            aligment:aligment.clone(),
            name:name.to_string(),
            d:d_,
        }
        // ()
        // (height,gap)
    }
    fn calc_d(name:&str,aligment:&Aligment,font:&str,val_above:f64,val_below:f64) -> SVG{

        let v: Vec<&str> = name.split('(').collect();
        let v2: Vec<&str>  = match v.get(1){
            None => "",
            Some(str) => *str,
        }.split(',').collect();
        let ans = match v[0]{
            "S" => {Self::calc_d_syl(v2[0],v2[1],v2[2],&v2[3][0..v2[3].len()-1],aligment,font,val_above,val_below)},
            "P" | "Pd" => {Self::calc_d_p(&v2[0][0..v2[0].len()-1],aligment,font,val_above,val_below,true)},
            "V" | "Vd" => {Self::calc_d_v(v2[0],&v2[1][0..v2[1].len()-1],aligment,font,val_above,val_below,true)},
            "C"  | "Cd" => {Self::calc_d_c(v2[0],&v2[1][0..v2[1].len()-1],aligment,font,val_above,val_below,true)},
            _ => {SVG::new("M0,0")},
        };
        ans
    }

    fn get_gpos(fullname:&str,alig:&Aligment,realm:&str,font:&str) -> (f64,f64,f64,f64){

        let mut ans = (0.0,0.0,0.0,0.0);
        let mut map = HashMap::new();

        let regex_datatable =  FontTable::get_datatable_unwraped("REGEX","");

        map.insert(String::from("Pattern_sub"), DataType::Char(String::from(realm)));
        map.insert(String::from("Pattern"), DataType::Regex(regex_datatable.get_single_data_from_name_unwraped(match alig{
            &Aligment::Vertical => "v|u",
            _ => "h|u",
        }).get_value_as_regex("Parsed")));

        let i = FontTable::get_datatable_unwraped("GPOS",font).get_data_unwraped(&map);
        
        for i_ in i {
            let p_detector = i_.get_value_as_regex("Parsed");
            match p_detector.captures(fullname){
                Some(_) => {
                    ans.0 += i_.get_value_as_f64("OffsetX0");
                    ans.1 += i_.get_value_as_f64("OffsetY0");
                    ans.2 += i_.get_value_as_f64("OffsetX1");
                    ans.3 += i_.get_value_as_f64("OffsetY1");
                },
                _ => {},
            }
        }
        ans
    }
    fn calc_d_syl(form:&str,onset:&str,vowel:&str,coda:&str,aligment:&Aligment,font:&str,val_above:f64,val_below:f64) -> SVG{

        let regex_datatable =  FontTable::get_datatable_unwraped("REGEX","");
        
        let fullname = String::from("S(") + form + "," + onset +"," + vowel +"," + coda +")";
        
        let (_,offset_x,_,offset_y) = Self::get_gpos(&fullname,aligment,"IN",font);
        
        let (mut d0,mut d1,mut d2) = (Self::calc_d_c("onset",onset,aligment,font,0.0,0.0,false),Self::calc_d_v(form,vowel,aligment,font,0.0,0.0,false),Self::calc_d_c("coda",coda,aligment,font,0.0,0.0,false));
        let (_,vowel_top,_,vowel_bottom) = d1.get_border();

        let (offset_x,offset_y) = Self::calc_off_syl(offset_x,offset_y,form,vowel,&mut d0,&mut d1,&mut d2);

        let _ = d1.merge_three(0.0,offset_x,d0,0.0,offset_y,d2); let _ = d1.to_rel();

        let (_,top,_,bottom) = d1.get_border();
        
        let p_detector = regex_datatable.get_single_data_from_name_unwraped("extract_name").get_value_as_regex("Parsed");
        let v_ = match p_detector.captures(vowel){
            None => "".to_string(),
            Some(i) => i[0].to_string(),
        };
        let mut map = HashMap::new();
        map.insert(String::from("Name"), DataType::Char(v_));
        let i = FontTable::get_datatable_unwraped("VOWEL",font).get_data_single_unwraped(&map);
        let isnull = i.get_value_as_bool("isNullVowel");
        
        let width = FontTable::get_datatable_unwraped("FONTDATA",font).get_single_data().get_value_as_f64("Thickness");
        match aligment{
            Aligment::Vertical => {
                if isnull {
                    match form{
                        "init" | "mid" | "fin" => {
                            let _ = d1.set_gap_alter(0.0,val_above+top);
                            let _ = d1.merge_two(0.0,0.0,SVG::build_skewer(-width/2.0,val_above+top,width/2.0,val_below+bottom));
                        },
                        _ => {
                            let _ = d1.set_gap_alter(0.0,val_below+bottom);
                            let _ = d1.set_gap(0.0,val_above+top);
                        }
                    }
                } else {
                    match form{
                        "init" => {
                            let _ = d1.set_gap(0.0,val_above);
                            let _ = d1.merge_two(0.0,0.0,SVG::build_skewer(-width/2.0,val_above+vowel_bottom,width/2.0,val_below+bottom));
                        },
                        "mid" => {
                            let _ = d1.merge_three(0.0,0.0,SVG::build_skewer(-width/2.0,val_above+top,width/2.0,vowel_top),0.0,0.0,SVG::build_skewer(-width/2.0,vowel_bottom,width/2.0,val_below+bottom));
                        },
                        "fin" => {
                            let _ = d1.merge_two(0.0,0.0,SVG::build_skewer(-width/2.0,val_above+top,width/2.0,vowel_top));
                        },
                        _ => {
                            let _ = d1.set_gap_alter(0.0,val_below+bottom);
                            let _ = d1.set_gap(0.0,val_above+top);
                        }
                    }
                }
            },
            _ => {
                let vt = (vowel_bottom - vowel_top)/2.0;
                let vt_b = (-bottom - top)/2.0;
                match form{
                    "init" => {
                        let _ = d1.set_gap(0.0,0.0);
                        let _ = d1.merge_two(0.0,0.0,SVG::build_skewer(-width/2.0,vowel_bottom,width/2.0,val_above));
                    },
                    "mid" => {
                        let _ = d1.set_gap(0.0,val_below+vt_b-vt);
                        let _ = d1.merge_three(0.0,0.0,SVG::build_skewer(-width/2.0,val_below+vt_b+vowel_top,width/2.0,val_above),0.0,0.0,SVG::build_skewer(-width/2.0,vowel_bottom,width/2.0,val_below+vt_b-vt));
                    },
                    "fin" => {
                        let _ = d1.set_gap(0.0,val_above-vowel_bottom);
                        let _ = d1.merge_two(0.0,0.0,SVG::build_skewer(-width/2.0,0.0,width/2.0,val_above-vowel_bottom));
                    },
                    _ => {
                        let _ = d1.set_gap(0.0,val_below-vt);
                    }
                }
            }
        }
        let _ = d1.to_rel();
        d1
    }
    fn calc_d_cvp(form:&str,name:&str,aligment:&Aligment,font:&str,gt:&GlyphType,val_above:f64,val_below:f64,is_isolated:bool) -> SVG{
        
        if name == "" {return SVG::build_m(0.0,0.0)};
        let n_: Vec<&str> = name.split('-').collect();
        let mut ans = SVG::build_m(0.0,0.0);
        let regex_datatable =  FontTable::get_datatable_unwraped("REGEX","");
        let p_detector = regex_datatable.get_single_data_from_name_unwraped("extract_name").get_value_as_regex("Parsed");
        
        let dt_a = FontTable::get_datatable_unwraped(match gt{
            GlyphType::Vowel | GlyphType::V_Diacritical => "DIACRITIC_V",
            GlyphType::Consonant | GlyphType::C_Diacritical => "DIACRITIC_C",
            GlyphType::Punctuation | GlyphType::P_Diacritical => "DIACRITIC_P",
            _ => ""
        },"");
        let label = match gt{
            GlyphType::Vowel => "V(",
            GlyphType::Consonant => "C(",
            GlyphType::Punctuation => "P(",
            GlyphType::V_Diacritical => "Vd(",
            GlyphType::C_Diacritical => "Cd(",
            GlyphType::P_Diacritical => "Pd(",
            _ => ""
        };

        let first_name = String::from("")+label+n_[0]+")";
        let offset_y_sub = if is_isolated && aligment != &Aligment::Vertical {
            let (_,_,_,gap) = Glyph::get_gpos(&first_name,aligment,"INEX",font);
            let height = FontTable::get_datatable_unwraped("FONTDATA",font).get_single_data().get_value_as_f64("Height");
            gap+height/2.
            } else {0.};

        let glyph_datatable = FontTable::get_datatable_unwraped("GLYPH",font);

        for n in (0..n_.len()).rev() {
            
            let last = n_[n];
            let last_name = match p_detector.captures(last){
                None => "".to_string(),
                Some(i) => i[0].to_string(),
            };
            let last_variant = &last[last_name.len()..];
            let mut last_fullname = last_name + "_" + form + "_" + last_variant;

            let i = match glyph_datatable.get_single_data_from_name(&last_fullname){
                Ok(some) => {some},
                Err(_) => {
                    glyph_datatable.get_single_data_from_name_unwraped(&"null")
                }
            };

            let mut svg = i.get_value_as_svg("Path");
            
            if n == n_.len()-1 {
                if n == 0 {
                let _ = svg.set_gap(0.0,offset_y_sub);
                let _ = svg.to_rel();
                }
                ans = svg;
            } else {
                let rest = n_[0..n+2].join("-");
                
                let rest_ = String::from("") + label + form + "," + &rest + ")";
                let (offset_x,offset_y,_,_) = Self::get_gpos(&rest_,aligment,"DIAC",font);

                let last_name_b = match p_detector.captures(n_[n+1]){
                    None => "".to_string(),
                    Some(i) => i[0].to_string(),
            };

            let i_ = dt_a.get_data_from_name_unwraped(&last_name_b);
            match i_.get(0){
                Some(i_) => {
                    let alig_x = i_.get_value_as_i32("x_Align");
                    let alig_y = i_.get_value_as_i32("y_Align");

                    let (offset_x,offset_y) = match gt{
                        GlyphType::Vowel => Self::calc_off_v(offset_x,offset_y,alig_x,alig_y,&mut svg),
                        GlyphType::Consonant => Self::calc_off_c(form,offset_x,offset_y,alig_x,alig_y,&mut svg),
                        GlyphType::Punctuation => Self::calc_off_v(offset_x,offset_y,alig_x,alig_y,&mut svg),
                        _ => (offset_x,offset_y)
                    };
                    let _ = svg.merge_two(offset_x,offset_y+ if n == 0 { offset_y_sub } else {0.0},ans);
                    let _ = svg.to_rel();
                    ans = svg;
                    },
                    None => {},
                }
            }
        }
        if is_isolated && aligment == &Aligment::Vertical{
            let (_,top,_,bottom) = ans.get_border();
            let _ = ans.set_gap_alter(0.0,val_below+bottom);
            let _ = ans.set_gap(0.0,val_above+top);
            let _ = ans.to_rel();
        }
        ans
    }


    fn calc_d_p(name:&str,aligment:&Aligment,font:&str,val_above:f64,val_below:f64,is_isolated:bool) -> SVG{
        Self::calc_d_cvp("",name,aligment,font,&GlyphType::Punctuation,val_above,val_below,is_isolated)
    }
    fn calc_d_v(form:&str,name:&str,aligment:&Aligment,font:&str,val_above:f64,val_below:f64,is_isolated:bool) -> SVG{
        Self::calc_d_cvp(form,name,aligment,font,&GlyphType::Vowel,val_above,val_below,is_isolated)
    }
    fn calc_d_c(form:&str,name:&str,aligment:&Aligment,font:&str,val_above:f64,val_below:f64,is_isolated:bool) -> SVG{
        Self::calc_d_cvp(form,name,aligment,font,&GlyphType::Consonant,val_above,val_below,is_isolated)
    }
    fn calc_off_c(form:&str,offset_x:f64,offset_y:f64,aligx:i32,aligy:i32,d0:&mut SVG) -> (f64,f64){
        let dir = if form == "onset" {1} else {-1};

        let (l0,t0,r0,b0) = d0.get_border();
       
        let x =  (l0*(1-aligx*dir)as f64/2.0+r0 *(1+aligx*dir)as f64/2.0)+offset_x*dir as f64;
        let y =  (t0*(1-aligy)as f64/2.0+b0 *(1+aligy)as f64/2.0)+offset_y;

        (x,y)
    }
    fn calc_off_v(offset_x:f64,offset_y:f64,aligx:i32,aligy:i32,d0:&mut SVG) -> (f64,f64){
        let (l0,t0,r0,b0) = d0.get_border();

        let x =  (l0*(1-aligx) as f64 /2.0+r0*(1+aligx) as f64 /2.0)+offset_x;
        let y =  (t0*(1-aligy) as f64 /2.0+b0*(1+aligy) as f64 /2.0)+offset_y;
        (x,y)
    }
    fn calc_off_syl(offset_x:f64,offset_y:f64,form:&str,vowel:&str, d0:&mut SVG, d1:&mut SVG, d2:&mut SVG) -> (f64,f64){
        let regex_datatable =  FontTable::get_datatable_unwraped("REGEX","");
        let p_detector = regex_datatable.get_single_data_from_name_unwraped("extract_name").get_value_as_regex("Parsed");

        let v_name = match p_detector.captures(vowel){
            None => "".to_string(),
            Some(i) => i[0].to_string(),
        };

        let i = FontTable::get_datatable_unwraped("VOWEL","").get_single_data_from_name_unwraped(&v_name);

        let (_,t0,_,b0) = d0.get_border();
        let (_,t1,_,b1) = d1.get_border();
        let (_,t2,_,b2) = d2.get_border();

        let top_gap :f64 = if d0.len() > 0{
            if d2.len() > 0 {f64::min(t0,t2)}
            else {t2}}
            else{ if d2.len() > 0 {t2} else {0.0} };
        let bottom_gap :f64 = if d0.len() > 0{
            if d2.len() > 0 {f64::max(b0,b2)}
            else {b2}}
            else{if d2.len() > 0 {b2}
            else {0.0}};
        
        if i.get_value("isNullVowel").to_bool() {
            return (b1+top_gap+offset_x,b1+top_gap+offset_y)
        } else {
            match form{
            "init" => return (b1 + top_gap + offset_x , b1 + top_gap + offset_y),
            "mid" | "sol" => return (t1 - b0 + offset_x , b1 + t2 + offset_y),
            "fin" => return (t1 - bottom_gap + offset_x , t1 - bottom_gap + offset_y),
            _ => return (0.0,0.0),
            }
        }
    }

}