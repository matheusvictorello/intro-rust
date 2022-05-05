fn main() {
    let op = Some(42);

    match op {
        Some(n) => {
            println!("Algum valor: {}", n);
        }, // Opcional
        None => {
            println!("Nenhum valor");
        },
    }
}