use std::collections::HashMap;

fn main() {
    let s = "Learn Rust with me me me learn with".to_lowercase();

    let mut count_map = HashMap::new();

    for i in s.split_whitespace() {//split_whitespace() sozleri boyleye yaramaktadir ve sozler arasindaki bosluklari bularak onlari o bosluklardan bolmektedir
        let count = count_map.entry(i).or_insert(0);
        *count+=1;
    }
    println!("{:?}",count_map);
}
