fn main() {
    let v = vec![0, 1, 2, 3, 4, 5];

    for i in v.iter() {
        println!("for {}", i);
    }

    println!("{:?}", v);
}