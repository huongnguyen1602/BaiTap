use std::any::Any;

use regex::Regex;
//use regex::bytes:Regex;
fn main() {
    let str="This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    let mut a = String::new();
    println!("Nhập chuỗi cần tìm: ");
    std::io::stdin().read_line(&mut a).unwrap();
    a.truncate(a.len()-1);
    let re = Regex::new(&a).unwrap();
    let result = re.find_iter(str).count();
    println!("{}",result);
}
