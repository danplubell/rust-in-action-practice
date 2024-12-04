fn isolate_sign_bit() -> u32{
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    n_bits >> 31
}
fn isolate_exponent() -> i32 {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let exponent_ = n_bits >>23; //erase the mantissa
    let exponent_ = exponent_ & 0xff; //erase the sign bit
    (exponent_ as i32) -  127 // subtract bias
    
    
}
fn isolate_mantissa() -> f32{
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let mut mantissa: f32 = 1.0;
    
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_-23.0);
            mantissa += weight;
        }
    }
    mantissa  
}
fn main() {
    println!("{}",isolate_sign_bit());
    println!("{}",isolate_exponent());
    println!("{}",isolate_mantissa());
}
