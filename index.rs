use std::thread;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use simple_user_input::get_input;

fn main() {

    println!("1 - Encriptar");
    println!("2 - Desencriptar");
    println!("3 - Fuerza bruta");
    let input: String = get_input("Seleccione una opcion...");
    if(input=="1"){
        let input_plain_text: String = get_input("Plain text: ");
        let input_key: String = get_input("Key: ");

        let mut permute_ten = permute_ten_key(convert_bool_key(&input_key));
        let mut first_key = get_first_key(permute_ten.as_mut());
        let mut second_key = get_second_key(permute_ten.as_mut());
        //encrypt
        let cipher_text = encrypt(convert_bool(&input_plain_text), first_key,second_key);
        let cipher_text_number: String = convert_string(cipher_text).into_iter().collect();
        println!("Ciphertext - {}", cipher_text_number);

    }else if(input=="2"){
        let input_plain_text: String = get_input("Ciphertext: ");
        let input_key: String = get_input("Key: ");

        let mut permute_ten = permute_ten_key(convert_bool_key(&input_key));
        let mut first_key = get_first_key(permute_ten.as_mut());
        let mut second_key = get_second_key(permute_ten.as_mut());
        //encrypt
        let cipher_text = encrypt(convert_bool(&input_plain_text),second_key, first_key);
        let cipher_text_number: String = convert_string(cipher_text).into_iter().collect();
        println!("Plain text - {}", cipher_text_number);
    }else if(input=="3"){
        let fine_name = "plainCipher.txt";

        // Open the path in read-only mode, returns `io::Result<File>`
        let file = File::open(fine_name).unwrap();

        let reader = BufReader::new(file);

        let mut keys = vec![];
        
        for (j, line) in reader.lines().enumerate() {
            
            let mut string = line.unwrap();
            let v: Vec<&str> = string.split('/').collect();
            let plain = convert_bool(v[0]);
            let cipher = convert_bool(v[1]);
            let mut i=0;
            if j == 0 { 
                while i<1024{
                    let ten_bit: [bool; 10] = generate_key(i);
                    let plain_text : [bool;8] = plain;
                    // KEYS
                    let mut permute_ten = permute_ten_key(ten_bit);
                    let mut first_key = get_first_key(permute_ten.as_mut());
                    let mut second_key = get_second_key(permute_ten.as_mut());
                    //encrypt
                    let cipher_text = encrypt(plain_text, first_key,second_key);
                    if(cipher_text == cipher){
                        keys.push(i);
                        // println!("Llave encontrada");
                        // println!("PLAIN - {:?}", plain_text);
                        // println!("CIPHER - {:?}", cipher_text);
                        // println!("Key - {:?}", generate_key(i));
                        // println!("Key - {:?}", i);
                        // println!("{}", string);
                        // println!("\n");
                        i=i+1;
                    }
                    i=i+1;
                }
            } else {
                let mut keys_to_remove = vec![];
                for (i, key) in keys.iter().enumerate() {
                    let ten_bit: [bool; 10] = generate_key(*key);
                    let plain_text : [bool;8] = plain;
                    //let cipher : [bool;8]= [false, true, true, true, false, false, false, false];
                    // KEYS
                    let mut permute_ten = permute_ten_key(ten_bit);
                    let mut first_key = get_first_key(permute_ten.as_mut());
                    let mut second_key = get_second_key(permute_ten.as_mut());
                    //encrypt
                    let cipher_text = encrypt(plain_text, first_key,second_key);
                    if(cipher_text != cipher){
                        keys_to_remove.push(i);
                    }
                }
                for (index, i) in keys_to_remove.iter().enumerate(){
                    keys.remove(*i-index);
                }
            }
        }
        // println!("{:?}", keys);
        // println!("{:?}", generate_key(keys[0]));
        let key_number: String = convert_string_key(generate_key(keys[0])).into_iter().collect();
        println!("Key - {}", key_number);
    }
    //generate_key(keys)
}

fn get_first_key(ten_bit: &mut[bool]) -> [bool; 8] {
    let (first_five, second_five) : (&mut[bool], &mut[bool]) = ten_bit.split_at_mut(5);
    first_five.rotate_left(1);
    second_five.rotate_left(1);
    let first_key = permute_eight_key(ten_bit);
    return first_key;
}

fn get_second_key(ten_bit: &mut[bool]) -> [bool; 8] {
    let (second_shift_first, second_shift_second) : (&mut[bool], &mut[bool]) = ten_bit.split_at_mut(5);
    second_shift_first.rotate_left(2);
    second_shift_second.rotate_left(2);
    let second_key = permute_eight_key(ten_bit);
    return second_key;
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

fn permute_eight_key(ten_bit: &mut[bool]) -> [bool; 8]{
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

fn permute_i_p_1(eigth_bit_plain_text: std::vec::Vec<bool>) -> [bool; 8]{
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

fn s0(first_four_xor: &mut[bool]) -> [bool; 2]{
    let mut s0: [bool;2] = [false;2];
    if(!first_four_xor[0]&&!first_four_xor[3]){
        if(!first_four_xor[1]&&!first_four_xor[2]){
            s0[1]=true;
        } else if(first_four_xor[1]&&!first_four_xor[2]){
            s0[0]=true;
            s0[1]=true;
        } else if(first_four_xor[1]&&first_four_xor[2]){
            s0[0]=true;
        }
    } else if(!first_four_xor[0]&&first_four_xor[3]){
        if(!first_four_xor[1]&&!first_four_xor[2]){
            s0[0]=true;
            s0[1]=true;
        } else if(!first_four_xor[1]&&first_four_xor[2]){
            s0[0]=true;
        } else if(first_four_xor[1]&&!first_four_xor[2]){
            s0[1]=true;
        } 
    } else if(first_four_xor[0]&&!first_four_xor[3]){
        if(!first_four_xor[1]&&first_four_xor[2]){
            s0[0]=true;
        } else if(first_four_xor[1]&&!first_four_xor[2]){
            s0[1]=true;
        } else if(first_four_xor[1]&&first_four_xor[2]){
            s0[0]=true;
            s0[1]=true;
        }
    } else if(first_four_xor[0]&&first_four_xor[3]){
        if(!first_four_xor[1]&&!first_four_xor[2]){
            s0[0]=true;
            s0[1]=true;
        } else if(!first_four_xor[1]&&first_four_xor[2]){
            s0[1]=true;
        } else if(first_four_xor[1]&&!first_four_xor[2]){
            s0[0]=true;
            s0[1]=true;
        } else if(first_four_xor[1]&&first_four_xor[2]){
            s0[0]=true;
        }
    }
    return s0;
}

fn s1(second_four_xor: &mut[bool]) -> [bool; 2]{
    let mut s1: [bool;2] = [false;2];
    if(!second_four_xor[0]&&!second_four_xor[3]){
        if(!second_four_xor[1]&&second_four_xor[2]){
            s1[1]=true;
        } else if(second_four_xor[1]&&!second_four_xor[2]){
            s1[0]=true;
        } else if(second_four_xor[1]&&second_four_xor[2]){
            s1[0]=true;
            s1[1]=true;
        }
    } else if(!second_four_xor[0]&&second_four_xor[3]){
        if(!second_four_xor[1]&&!second_four_xor[2]){
            s1[0]=true;
        } else if(second_four_xor[1]&&!second_four_xor[2]){
            s1[1]=true;
        } else if(second_four_xor[1]&&second_four_xor[2]){
            s1[0]=true;
            s1[1]=true;
        }
    } else if(second_four_xor[0]&&!second_four_xor[3]){
        if(!second_four_xor[1]&&!second_four_xor[2]){
            s1[0]=true;
            s1[1]=true;
        } else if(second_four_xor[1]&&!second_four_xor[2]){
            s1[1]=true;
        }
    } else if(second_four_xor[0]&&second_four_xor[3]){
        if(!second_four_xor[1]&&!second_four_xor[2]){
            s1[0]=true;
        } else if(!second_four_xor[1]&&second_four_xor[2]){
            s1[1]=true;
        } else if(second_four_xor[1]&&second_four_xor[2]){
            s1[0]=true;
            s1[1]=true;
        }
    }
    return s1;
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

fn encrypt(plain_text: [bool;8], first_key:[bool;8], second_key:[bool;8]) -> [bool; 8]{
    let mut permute_ip_plain = permute_i_p(plain_text);
    let permute_ip_plain_string: String = convert_string(permute_ip_plain).into_iter().collect();
    println!("IP - {}", permute_ip_plain_string);
    let (first_four_plain, second_four_plain) : (&mut[bool], &mut[bool]) = permute_ip_plain.split_at_mut(4);
    let mut function_first_key = fkey(first_four_plain, second_four_plain, first_key);
    let function_first_key_string: String = convert_string_fkey(function_first_key).into_iter().collect();
    println!("fKey1 - {}", function_first_key_string);
    // ROUND 2
    let sw: String = convert_vect_string_eigth([second_four_plain.as_mut(), function_first_key.as_mut()].concat()).into_iter().collect();
    println!("SW - {}", sw);
    let mut function_second_key = fkey(second_four_plain, function_first_key.as_mut(), second_key);
    let function_sec_key_string: String = convert_string_fkey(function_second_key).into_iter().collect();
    println!("fKey2 - {}", function_sec_key_string);
    let pre_ip_1: String = convert_vect_string_eigth([function_second_key.as_mut(), function_first_key.as_mut()].concat()).into_iter().collect();
    println!("pre_ip_1 - {}", pre_ip_1);
    let cipher = permute_i_p_1([function_second_key.as_mut(), function_first_key.as_mut()].concat());
    return cipher;
}

fn fkey(first_plain: &mut[bool], sec_plain: &mut[bool], key:[bool;8]) -> [bool; 4] {
    let e_p_plain = permute_e_p(sec_plain);
    let mut xor_ep_first_key = xor_eigth_bits(e_p_plain, key);
    let (first_four_xor, second_four_xor) : (&mut[bool], &mut[bool]) = xor_ep_first_key.split_at_mut(4);
    let mut s0_r1 = s0(first_four_xor);
    let mut s1_r1 = s1(second_four_xor);
    let p4_plain = permute_p4([s0_r1, s1_r1].concat());
    let mut first_sw = xor_four_bits(p4_plain, first_plain);
    return first_sw;
}

fn convert_bool(string: &str) -> [bool; 8] {
    let mut arr: [bool;8]=[false;8];
    for (i, a) in string.chars().enumerate() {
        if a == '1' {
            arr[i] = true;
        } else {
            arr[i] = false;
        }
    }
    return arr;
}

fn convert_bool_key(string: &str) -> [bool; 10] {
    let mut arr: [bool;10]=[false;10];
    for (i, a) in string.chars().enumerate() {
        if a == '1' {
            arr[i] = true;
        } else {
            arr[i] = false;
        }
    }
    return arr;
}

fn convert_string(text:[bool; 8]) ->  [char; 8] {
    let mut arr: [char; 8] =['0';8];
    for x in 0..8{
        if(text[x]==true){
            arr[x]='1'
        }else{
            arr[x]='0'
        }
    }
    return arr;
}

fn convert_string_key(text:[bool; 10]) ->  [char; 10] {
    let mut arr: [char; 10] =['0';10];
    for x in 0..8{
        if(text[x]==true){
            arr[x]='1'
        }else{
            arr[x]='0'
        }
    }
    return arr;
}

fn generate_key(key:u32) -> [bool; 10]{
    let mut temp_key: [bool; 10] = [false;10];
    let x = key;
    let y = format!("{:b}", x);
    let mut w : Vec<char> = y.chars().collect();
    while(w.len() < 10){
        w.insert(0,'0');
    }
    for x in 0..10 {
        if(w[x]=='1'){
            temp_key[x] = true;
        }
    }
    return temp_key;
}


mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}

fn convert_string_fkey(text:[bool; 4]) ->  [char; 4] {
    let mut arr: [char; 4] =['0';4];
    for x in 0..4{
        if(text[x]==true){
            arr[x]='1'
        }else{
            arr[x]='0'
        }
    }
    return arr;
}

fn convert_vect_string_eigth(vector: std::vec::Vec<bool>) -> [char; 8]{
    let mut arr: [char; 8] =['0';8];
    for x in 0..8{
        if(vector[x]==true){
            arr[x]='1'
        }else{
            arr[x]='0'
        }
    }
    return arr;
}