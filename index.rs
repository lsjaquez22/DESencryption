fn main() {
    let ten_bit: [bool; 10] = [true, true, false, false, false, true, true, true, true, false];
    let plain_text : [bool;8] = [false, false, true, false, true, false, false, false];
    let mut permute_ten = permute_ten_key(ten_bit);
    let (first_five, second_five) : (&mut[bool], &mut[bool]) = permute_ten.split_at_mut(5);
    first_five.rotate_left(1);
    second_five.rotate_left(1);
    let first_key = permute_eight_key(permute_ten);

    let (second_shift_first, second_shift_second) : (&mut[bool], &mut[bool]) = permute_ten.split_at_mut(5);
    second_shift_first.rotate_left(2);
    second_shift_second.rotate_left(2);

    let second_key = permute_eight_key(permute_ten);

    let mut permute_ip_plain = permute_i_p(plain_text);
    let (first_four_plain, second_four_plain) : (&mut[bool], &mut[bool]) = permute_ip_plain.split_at_mut(4);

    let e_p_plain = permute_e_p(second_four_plain);

    let mut xor_ep_first_key = xor_eigth_bits(e_p_plain, first_key);
    let (first_four_xor, second_four_xor) : (&mut[bool], &mut[bool]) = xor_ep_first_key.split_at_mut(4);
    println!("PRUEBA - {:?}", first_four_xor);
    println!("PRUEBA2 - {:?}", second_four_xor);

    // println!("PRUEBA P4 - {:?}", permute_p4([true, false, false, false]));
    // println!("PRUEBA XOR 4 - {:?}", xor_four_bits([false, false, false, true], [false, false, true, false]));
    // println!("PRUEBA IP-1 - {:?}", permute_i_p_1([false, false, false, true, false, false, true, true]));
    // println!("PRUEBA CONCATENET - {:?}", [first_four_xor, second_four_xor].concat());
    
    // println!("{:?}",first_key);
    // println!("{:?}", second_key);

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
    // println!("{:?}", ten_bit);
    let p8: [usize; 8] = [6,3,7,4,8,5,10,9];
    let mut i = 0;
    for x in p8.iter(){
        new_8_key[i] = ten_bit[*x-1];
        i+=1;
    }
    return new_8_key;
}

fn permute_e_p(four_bit_plain_text: &mut[bool]) -> [bool; 8]{
    let mut new_e_p: [bool; 8] = [false;8];
    // println!("{:?}", four_bit_plain_text);
    let e_p: [usize; 8] = [4,1,2,3,2,3,4,1];
    let mut i = 0;
    for x in e_p.iter(){
        new_e_p[i] = four_bit_plain_text[*x-1];
        i+=1;
    }
    return new_e_p;
}

fn permute_i_p(eigth_bit_plain_text: [bool; 8]) -> [bool; 8]{
    let mut new_i_p: [bool; 8] = [false;8];
    // println!("{:?}", eigth_bit_plain_text);
    let i_p: [usize; 8] = [2,6,3,1,4,8,5,7];
    let mut i = 0;
    for x in i_p.iter(){
        new_i_p[i] = eigth_bit_plain_text[*x-1];
        i+=1;
    }
    return new_i_p;
}

fn permute_i_p_1(eigth_bit_plain_text: [bool; 8]) -> [bool; 8]{
    let mut new_i_p_1: [bool; 8] = [false;8];
    // println!("{:?}", eigth_bit_plain_text);
    let i_p_1: [usize; 8] = [4,1,3,5,7,2,8,6];
    let mut i = 0;
    for x in i_p_1.iter(){
        new_i_p_1[i] = eigth_bit_plain_text[*x-1];
        i+=1;
    }
    return new_i_p_1;
}

fn permute_p4(four_bit_plain_text: [bool; 4]) -> [bool; 4]{
    let mut new_p4: [bool; 4] = [false;4];
    // println!("{:?}", four_bit_plain_text);
    let p4: [usize; 4] = [2,4,3,1];
    let mut i = 0;
    for x in p4.iter(){
        new_p4[i] = four_bit_plain_text[*x-1];
        i+=1;
    }
    return new_p4;
}

fn xor_eigth_bits(eigth_bits : [bool; 8], key_bits : [bool; 8]) -> [bool; 8] {
    let mut new_xor_eight: [bool; 8] = [false;8];
    // println!("{:?}", four_bit_plain_text);
    let mut i = 0;
    for x in key_bits.iter(){
        new_xor_eight[i] = eigth_bits[i] ^ key_bits[i];
        i+=1;
    }
    return new_xor_eight;
}

fn xor_four_bits(first_four_bits : [bool; 4], second_four_bits : [bool; 4]) -> [bool; 4] {
    let mut new_xor_four: [bool; 4] = [false;4];
    // println!("{:?}", four_bit_plain_text);
    let mut i = 0;
    for x in first_four_bits.iter(){
        new_xor_four[i] = first_four_bits[i] ^ second_four_bits[i];
        i+=1;
    }
    return new_xor_four;
}

