use std::fmt;

struct Livro {
    titulo:  String,
    paginas: u32,
}

impl fmt::Display for Livro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "O livro {:?} com {} página(s)", self.titulo, self.paginas)
    }
}

fn main() {
    let livro = Livro {
        titulo:  String::from("Rust"),
        paginas: 500,
    };

    println!("{}", livro);
}