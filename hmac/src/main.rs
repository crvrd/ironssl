//extern crate rand;

extern crate shaman;
//extern crate rustc_serialize;

use self::shaman::digest::Digest;
//use rustc_serialize::hex;

static BLOCK_SIZE: usize = 64; // since we're using sha256, block size is 32

fn hmac (key : &mut Vec<u8>, message: &mut Vec<u8>) -> Vec<u8> {

	let mut key_length = key.len();

	let mut sha = shaman::sha2::Sha256::new();

	// hash the key and truncate if it's too long
	if key_length > BLOCK_SIZE/2
	{
		sha.input(key.as_ref());
		let mut key_digest : [u8; 64] = [0; 64];
		sha.result(&mut key_digest);

		// need to copy this into key
		for _i in 0..BLOCK_SIZE
		{
			key[_i] = key_digest[_i];
		}

		// truncate key to only BLOCK_SIZE
		key.truncate(BLOCK_SIZE);

		// Need this because usize can't be negative 
		// when we do key.reserve()
		key_length = BLOCK_SIZE;
	}

	// pad the key if it's too small
	// need to resize key to 64 first
	key.reserve(BLOCK_SIZE - key_length);
	if key_length < BLOCK_SIZE
	{
		for _i in key.len()..BLOCK_SIZE {
		   key.push(0); //zero pad the key if it is less than BLOCK_SIZE
		}
	}

	let mut o_keys: Vec<u8> = Vec::new();    
	let mut i_keys: Vec<u8> = Vec::new();
	for i in 0..BLOCK_SIZE {
		o_keys.push(0x5c ^ key[i]);
		i_keys.push(0x36 ^ key[i]); 
	}
   
	let mut inner: Vec<u8> = Vec::new();

	// have to append manually because rust sucks

	// append i_keys manually
	for x in 0..BLOCK_SIZE
	{
		inner.push(i_keys[x]);
	}
	// concatenate message manually
	for y in message.iter()
	{
		inner.push(*y);
	}

	// hash this concatenation of i_key and message
	sha.reset();
	sha.input(inner.as_ref());
	let mut inner_hash_digest : [u8; 64] = [0; 64];

   /* for a in 0..BLOCK_SIZE
    {
        inner_hash_digest.push(0);
    }*/

	sha.result(&mut inner_hash_digest);
	// copy and truncate
	for _i in 0..BLOCK_SIZE
	{
		inner[_i] = inner_hash_digest[_i];
	}
	inner.truncate(BLOCK_SIZE/2);
	// Now concatenate o_keys + inner
	for z in inner.iter()
	{
		o_keys.push(*z);
	}

	// hash this contenation, outputting to ret_val
	let mut ret_val: Vec<u8> = Vec::new();

	sha.reset();
	sha.input(o_keys.as_ref());
	let mut hmac_digest : [u8; 64] = [0; 64];
	sha.result(&mut hmac_digest);

	// copy and truncate
	for _i in 0..BLOCK_SIZE
	{
		ret_val.push(hmac_digest[_i]);
	}
	ret_val.truncate(BLOCK_SIZE/2);

	return ret_val;
}


fn main () {

	// Command line input doesn't want to work, so try and figure out a way for user input similar to scanf

    let mut key: Vec<u8> = b"gkeythisisaverylongkeythisisaverylongkeygkeythisisaverylongkeythisisaverylongkey".to_vec();
    let mut message: Vec<u8> = b"thisisatest".to_vec();
	let ret = hmac(&mut key, &mut message);

	//print as hex string
	for i in ret.iter()
	{
		print!("{:02x}", i);
 //       let formatted = format!("{:.*}",1,i);
//		println!("{}", formatted);
	}
    print!("\n");

}
