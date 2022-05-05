fn main() {
    let op = Some(42);

    if let Some(n) = op {
        println!("Algum valor: {}", n);
    } else {
        println!("Nenhum valor");
    }
}