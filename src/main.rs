
fn main() {
    //VECTORS
    let v: Vec<i32> = Vec::new();
    let mut a = [1, 2, 4];
    let mut v = vec![1, 2, 4];

    v.push(5);
    v.push(6);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let fourth: &i32 = &v[3];
    println!("The fourth element is {}", fourth);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There's not third element. "),
    }

    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}

