//extern crate rust-crypto;


fn hmac (key: &mut Vec<u8>, message: &mut Vec<u8>) -> Vec<u8> {
    let block_size = 64;

    if key.len() < block_size {
        for i in key.len()..block_size {
           key[i] = 0; //zero pad the key if it is less than block_size
        }
    }
    if key.len() > block_size {
        //if key len is greater than block size, hash the key
        //key = hash(key);
    } 
/*
    let mut o_keys:[u8, ..64];
    let mut i_keys:[u8, ..64];
    for i in 0..block_size {
        o_keys[i] = 0x5c ^ key[i];
        i_keys[i] = 0x36 ^ key[i]; 
    }
*/
    let mut o_keys: Vec<u8> = Vec::new();    
    let mut i_keys: Vec<u8> = Vec::new();
    for i in 0..block_size {
        o_keys.push(0x5c ^ key[i]);
        i_keys.push(0x36 ^ key[i]); 
    }
   
    let mut ret_val: Vec<u8> = Vec::new();
    ret_val.append(&mut i_keys);
    ret_val.append(&mut message);
    //hash that puppy
    ret_val.append(&mut o_keys);
    return ret_val
}


fn main () {
    let mut key: Vec<u8> = b"ghandi".to_vec();
    let mut message: Vec<u8> = b"hello".to_vec();
    hmac(&mut key, &mut message);

}
