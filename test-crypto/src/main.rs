// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate crypto;
extern crate rand;
extern crate cryptoimpl;

use crypto::{ symmetriccipher, buffer, aes, blockmodes, scrypt };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

use rand::Rng;

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize128,
            key,
            iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

fn main() {

    /*****************************************************
                            AES 
    *****************************************************/
    let message = "Cry Havoc, and let slip the dogs of war";
    let password = "testpass";

    println!("\nRunning AES test...");

    let mut key: [u8; 16] = [0; 16];
    let mut salt: [u8; 16] = [0; 16];
    let mut iv: [u8; 16] = [0; 16];

    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut iv);
    rng.fill_bytes(&mut salt);

    let sparams = scrypt::ScryptParams::new(4, 5, 6);

    scrypt::scrypt(password.as_bytes(), &salt, &sparams, &mut key);

    let reference_enc = encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
    let encrypted_data = cryptoimpl::aes::cbc_encrypt(message.as_bytes(), &key, &iv);

    assert!(&reference_enc[..] == &encrypted_data[..]);
    println!("AES test completed successfully");

    /*****************************************************
                        Diffie-Helman
    *****************************************************/

    println!("\nRunning Diffie Helman test...");

    // pre-shared info
    let g = 5;
    let p = 23;

    // alice's info
    let a_key = 6;
    let a_exp = cryptoimpl::dh::modexp(g, a_key, p); // sends to bob

    // bob's info
    let b_key = 15;
    let b_exp = cryptoimpl::dh::modexp(g, b_key, p);// sends to alice

    let shared_1 = cryptoimpl::dh::compute_shared_key(p, g, a_key, b_exp);
    let shared_2 = cryptoimpl::dh::compute_shared_key(p, g, b_key, a_exp);

    assert!(shared_1 == shared_2);
    println!("Diffie-Helman test completed successfully");

    /*****************************************************
                             HMAC
    *****************************************************/
    println!("\nRunning HMAC test...");

    let mut key: Vec<u8> = b"testkey".to_vec();
    let mut message: Vec<u8> = b"testmsg".to_vec();
    let ret = cryptoimpl::hmac::hmac(&mut key, &mut message);

    println!("Result of HMAC test:");

    //print as hex string
    for i in ret.iter()
    {
        print!("{:x}", i);
    }
    println!("");

}
