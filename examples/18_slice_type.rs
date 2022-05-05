fn main() {
    let s0: &str = "Matheus Victorello";
    let s1: &str = &s0[0..7];

    println!("{}", s1);

    let a0:  [u32; 5] = [0, 1, 2, 3, 4];
    let a1: &[u32]    = &a0[2..4];

    println!("{:?}", a1);
}