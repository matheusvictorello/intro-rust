trait Dev {
    fn desenvolver(&mut self, projeto: &str);
}

struct DevRust {
    desenvolvendo: bool
}

impl DevRust {
    fn new() -> Self {
        Self {
            desenvolvendo: false
        }
    }
}

impl Dev for DevRust {
    fn desenvolver(&mut self, _projeto: &str) {
        self.desenvolvendo = true;
    }
}

fn main() {
    let mut dev = DevRust::new();
    
    dev.desenvolver("Apresentação");

    println!("{}", dev.desenvolvendo);
}