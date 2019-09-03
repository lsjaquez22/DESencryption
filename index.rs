fn main() {
    let ten_bit: [bool; 10] = [true, false, true, false, false, false, false, false, true, false];
    let mut permute_ten = permute_ten_key(ten_bit);
    let (first_five, second_five) : (&mut[bool], &mut[bool]) = permute_ten.split_at_mut(5);
    first_five.rotate_left(1);
    second_five.rotate_left(1);
    let permute_eight = permute_eight_key(permute_ten);
    println!("{:?}",permute_eight);
    //let bit_key = key_to_string();
    // let mut h1 = String::from(&bit_key[0..5]);
    // let mut h2 = String::from(&bit_key[5..10]);
    // let mut firstElement = String::from(&h1[0..1]);
    // h1.push_str(&firstElement);
    // let mut h1_without = String::from(&h1[1..]);
    // firstElement = String::from(&h2[0..1]);
    // h2.push_str(&firstElement);
    // let mut h2_without = String::from(&h2[1..]);
    // println!("{}",h1_without);
    // println!("{}",h2_without);
    

}

fn permute_ten_key(ten_bit: [bool; 10]) -> [bool; 10]{
    let mut new_10_key: [bool; 10] = [false;10];
    let p10: [usize; 10] = [3,5,2,7,4,10,1,9,8,6];
    let mut i = 0;
    for x in p10.iter(){
        new_10_key[i] = ten_bit[*x-1];
        i+=1;
    }
    return new_10_key;
}

fn permute_eight_key(ten_bit: [bool; 10]) -> [bool; 8]{
    let mut new_8_key: [bool; 8] = [false;8];
    println!("{:?}", ten_bit);
    let p8: [usize; 8] = [6,3,7,4,8,5,10,9];
    let mut i = 0;
    for x in p8.iter(){
        new_8_key[i] = ten_bit[*x-1];
        i+=1;
    }
    return new_8_key;
}