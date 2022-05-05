fn main() {
    let u0:  u32 =  42;
    let u1: &u32 = &u0;
    let u2:  u32 = *u1;

    println!("{} {} {}", u0, u1, u2);
}