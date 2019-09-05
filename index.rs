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

    let new_first_four_xor= permute_s(first_four_xor);
    let new_second_four_xor = permute_s(second_four_xor);

    let p4_plain = permute_p4([s0(new_first_four_xor), s1(new_second_four_xor)].concat());
    

    let mut first_sw = xor_four_bits(p4_plain, second_four_plain);


    // ROUND 2

    let e_p_plain_r2 = permute_e_p(first_sw.as_mut());

    println!("PRUEBA FIRST SW - {:?}", first_sw);
    println!("PRUEBA H1 - {:?}", second_four_plain);

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

fn permute_p4(four_bit_plain_text: std::vec::Vec<bool>) -> [bool; 4]{
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

fn xor_four_bits(first_four_bits : [bool; 4], second_four_bits : &mut [bool]) -> [bool; 4] {
    let mut new_xor_four: [bool; 4] = [false;4];
    // println!("{:?}", four_bit_plain_text);
    let mut i = 0;
    for x in first_four_bits.iter(){
        new_xor_four[i] = first_four_bits[i] ^ second_four_bits[i];
        i+=1;
    }
    return new_xor_four;
}

fn s0(first_four_xor: [bool; 4]) -> [bool; 2]{
    let s0: [[bool;2];16] = [[false, true], [false, false], [true, true], [true, false], [true, true], [true, false], [false, true], [false, false], [false, false], [true, false], [false, true], [true, true], [true, true], [false, true], [false, false], [true, false]];
    let mut pos=0;
    if(first_four_xor[0] == true){
        pos=pos+8
    }
    if(first_four_xor[1] == true){
        pos=pos+4
    }
    if(first_four_xor[2] == true){
        pos=pos+2
    }
    if(first_four_xor[3] == true){
        pos=pos+1
    }
    return s0[pos]
}

fn s1(second_four_xor: [bool; 4]) -> [bool; 2]{
    let s1: [[bool;2];16] = [[false, false], [false, true], [true, false], [true, true], [true, false], [false, false], [false, true], [true, true], [true, true], [false, false], [false, true], [false, false], [true, false], [false, true], [false, false], [true, true]];
    let mut posS1=0;
    if(second_four_xor[0] == true){
        posS1=posS1+8
    }
    if(second_four_xor[1] == true){
        posS1=posS1+4
    }
    if(second_four_xor[2] == true){
        posS1=posS1+2
    }
    if(second_four_xor[3] == true){
        posS1=posS1+1
    }
    // println!("pos - {:?}", posS1);
    return s1[posS1]
}

fn permute_s(second_four_xor: &mut [bool]) -> [bool; 4]{
    let mut new_permute_s: [bool; 4] = [false;4];
    let p10: [usize; 4] = [1,4,2,3];
    let mut i = 0;
    for x in p10.iter(){
        new_permute_s[i] = second_four_xor[*x-1];
        i+=1;
    }
    return new_permute_s;
}


