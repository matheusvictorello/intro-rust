fn main() {
    let op = Some(42);

    let s = if let Some(n) = op {
        format!("Algum valor: {}", n)
    } else {
        format!("Nenhum valor")
    };

    println!("{}", s);
}