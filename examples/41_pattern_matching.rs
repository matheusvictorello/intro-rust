fn main() {
    let op = Some(42);

    match op {
        Some(42) => println!("É 42!"),
        Some(n)  => println!("Algum valor: {}", n),
        None     => println!("Nenhum valor"),
    }
}