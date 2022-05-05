fn main() {
    // let byte: i8   = -b'a';
    let bin:  i16  = -0b11;
    let oct:  i32  = -0o77;
    let hex:  i64  = -0xff;
    let dec:  i128 = -1_000;

    println!("{} {} {} {}", bin, oct, hex, dec);
}