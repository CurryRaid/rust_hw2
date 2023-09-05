pub fn inc(v: &Vec<char>) -> Vec<char> {
    let new = v
        .into_iter()
        .map(|x| std::char::from_u32((*x as u32) + 1).unwrap())
        .collect();
    new
}
