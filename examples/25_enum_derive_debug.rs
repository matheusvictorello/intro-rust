#[derive(Debug)]
enum Cor {
    RGB(u8, u8, u8),
    HSL{
        h: f32,
        s: f32,
        l: f32,
    }
}

fn main() {
    let c0 = Cor::RGB(255, 0, 0);
    let c1 = Cor::HSL {
        h: 135.0,
        s:   1.0,
        l:   0.5,
    };

    println!("{:?}", c0);
    println!("{:?}", c1);
}