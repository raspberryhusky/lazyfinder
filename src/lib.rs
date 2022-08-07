use regex::Regex;
use std::fs;
use std::error::Error;
use colored::Colorize;

pub struct FileMatcher<'a>{
    pub filename:&'a String,
    pub keys:&'a Vec<String>,
    pub is_reg_word:&'a bool,
}

impl<'a>  FileMatcher<'a> {
    pub fn new(filename:&'a String,k:&'a Vec<String>,iw:&'a bool) -> FileMatcher<'a>{
        let keys = k;
        let is_reg_word = iw;
        FileMatcher{filename,keys,is_reg_word}
    }
}
pub fn reg_search(i:&str,rm:&str,contents:&str) -> (){
    let re = Regex::new(&rm).unwrap();
    if re.is_match(contents){
        println!("{} : {} ","path".blue(),&i);
        let mut count = 0;
        for i in contents.lines(){
        count+=1;
        for j in re.find_iter(i){
            println!("\t{} {}: {}","line".red(),count.to_string().green(),j.as_str())
        }
    }
    }else{
        return ;
    }
}

pub fn normal_search(i:&str,key:&str,contents:&str)->(){
    if contents.contains(key){
        println!("{} : {} ","path".blue(),&i);
        let mut count = 0;
        for l in contents.lines(){
            count+=1;
            if l.contains(key){
                println!("\t{} {}: {}","line".red(),count.to_string().green(),l)
            }
        }
    }else{
        return ;
    }
}


pub fn run(file_matcher:FileMatcher) -> Result<(),Box<dyn Error>>{

    if file_matcher.is_reg_word.clone(){

        for j in file_matcher.keys.iter(){
            match  fs::read_to_string(file_matcher.filename.clone()){
                Ok(contents)=>reg_search(&file_matcher.filename,&j, &contents),
                _=>()
            }
        }

    }else {
            for j in file_matcher.keys.iter(){
                match  fs::read_to_string(file_matcher.filename.clone()){
                    Ok(contents)=>normal_search(&file_matcher.filename,&j, &contents),
                    _=>()
                }
            }
    }
    Ok(())
}

