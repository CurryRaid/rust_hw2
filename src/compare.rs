pub fn compare_string(x: &str, y: &str) -> bool {
    let chars1: Vec<char> = x.chars().collect();
    let chars2: Vec<char> = y.chars().collect();
    let len_x = chars1.len();
    let len_y = chars2.len();
    let len_min = if len_x > len_y { len_y } else { len_x };
    for i in 0..len_min {
        if chars1[i] > chars2[i] {
            return true;
        } else if chars1[i] < chars2[i] {
            return false;
        }
    }
    if len_x > len_y {
        true
    } else {
        false
    }
}
