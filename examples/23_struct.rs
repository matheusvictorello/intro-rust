// Não compila
struct Livro {
    titulo:  String,
    paginas: u32,
}

fn main() {
    let livro = Livro {
        titulo:  String::from("Rust"),
        paginas: 500,
    };

    println!("{:?}", livro);
}