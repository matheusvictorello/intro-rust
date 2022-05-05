fn main() {
    let byte: u8   =  b'a';
    let bin:  u16  =  0b11;
    let oct:  u32  =  0o77;
    let hex:  u64  =  0xff;
    let dec:  u128 =  1_000;

    println!("{} {} {} {} {}", byte, bin, oct, hex, dec);
}