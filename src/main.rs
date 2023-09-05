mod buffer;
mod closure;
mod compare;
fn main() {
    //test for buffer
    println!("test for buffer...");
    let a = buffer::Buffer { x: vec![1, 2, 3] };
    let b = buffer::Buffer {
        x: vec![1.3, 2.0, 3.6],
    };
    println!("1+2+3={}", a.sum());
    println!("1.3+2.0+3.6={}", b.sum());

    //test for compare
    println!("\ntest for compare...");
    let str1 = "abcd";
    let str2 = "abc";
    println!(
        "{} > {} is {}",
        str1,
        str2,
        compare::compare_string(str1, str2)
    );
    let str1 = "abc";
    let str2 = "bcd";
    println!(
        "{} > {} is {}",
        str1,
        str2,
        compare::compare_string(str1, str2)
    );

    let str1 = "abc";
    let str2 = "abc";
    println!(
        "{} > {} is {}",
        str1,
        str2,
        compare::compare_string(str1, str2)
    );
    println!("\ntest for closure...");
    let v0 = vec!['a', 'b', 'c', 'd', 'e'];
    let v1 = closure::inc(&v0);
    println!("old={:?}", v0);
    println!("new={:?}", v1);
}
