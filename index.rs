fn main() {
    let p8: [i32; 8] = [6,3,7,4,8,5,10,9];
    let bit_key = "1010101010";
    let mut h1 = String::from(&bit_key[0..5]);
    let mut h2 = String::from(&bit_key[5..10]);
    let mut firstElement = String::from(&h1[0..1]);
    h1.push_str(&firstElement);
    let mut h1_without = String::from(&h1[1..]);
    firstElement = String::from(&h2[0..1]);
    h2.push_str(&firstElement);
    let mut h2_without = String::from(&h2[1..]);
    let p8 = &h1_without;
    p8.push_str(&h2_without);
    println!("{}",p8);
    println!("{}",h1_without);
    println!("{}",h2_without);
    

}