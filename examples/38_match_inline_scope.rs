fn main() {
    let op = Some(42);

    let s = match op {
        Some(n) => format!("Algum valor: {}", n), // Obrigatório
        None    => format!("Nenhum valor"),
    };

    println!("{}", s);
}