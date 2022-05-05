fn main() {
    let op = Some(42);

    match op {
        Some(n) if n > 42 => println!("É maior que 42!"),
        Some(n)           => println!("Algum valor: {}", n),
        None              => println!("Nenhum valor"),
    }
}