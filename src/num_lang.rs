pub fn get_name(num: u32) -> String {
    let digits: i32 = format!("{}", num).len() as i32;
    match digits {
        3 => "hundred".to_string(),
        4 => "thousand".to_string(),
        5 => "ten thousand".to_string(),
        6 => "hundred thousand".to_string(),
        7 => "million".to_string(),
        8 => "ten million".to_string(),
        9 => "hundred million".to_string(),
        10 => "billion".to_string(),
        11 => "ten billion".to_string(),
        12 => "hundred billion".to_string(),
        13 => "trillion".to_string(),
        14 => "ten trillion".to_string(),
        15 => "hundred trillion".to_string(),
        16 => "quadrillion".to_string(),
        _ => "ERR".to_string(),
    }
}
