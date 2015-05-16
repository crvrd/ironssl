// act as a client, running tests
mod hmac;
mod dh;
mod aes;

extern crate rand;
extern crate crypto;

use crypto::scrypt;
use rand::Rng;

fn main() {

	println!("");

	/* TEST HMAC */
	{
		println!("Testing HMAC...\n");
		let mut hmac_key: Vec<u8> = b"gkeythisisaverylongkeythisisaverylongkeygkeythisisaverylongkeythisisaverylongkey".to_vec();
		let mut hmac_message: Vec<u8> = b"thisisatest".to_vec();
		let ret = hmac::hmac(&mut hmac_key, &mut hmac_message);

		//print as hex string
		for i in ret.iter()
		{
			print!("{:x}", i);
		}
		println!("\n");
	}
	
	/* TEST DH */
	{
		println!("Testing Diffie-Hellman...\n");

		// pre-shared info
		let g = 5;
		let p = 23;

		// alice and bob's (randomly generated) private keys
		let a_key = 6;
		let b_key = 15;

		dh::demo_dh(p, g, a_key, b_key);
	}

	/* TEST AES */
	{
		let aes_message = "Cry Havoc, and let slip the dogs of war";
		let aes_password = "testpass";

		let mut aes_key: [u8; 16] = [0; 16];
		let mut aes_salt: [u8; 16] = [0; 16];
		let mut aes_iv: [u8; 16] = [0; 16];

		let mut rng = rand::thread_rng();
		rng.fill_bytes(&mut iv);
		rng.fill_bytes(&mut salt);

		let sparams = scrypt::ScryptParams::new(4, 5, 6);

		scrypt::scrypt(password.as_bytes(), &salt, &sparams, &mut key);

		let encrypted_data = aes::cbc_encrypt(message.as_bytes(), &key, &iv);
		let decrypted_data = aes::cbc_decrypt(encrypted_data.as_bytes(), &key, &iv);

		println!("AES Encrypted Data: {:?}", encrypted_data);
		println!("AES Decrypted Data: {:?}", decrypted_data);
	}
}
