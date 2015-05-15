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
    let message = "Cry Havoc, and let slip the dogs of war";
    let password = "testpass";

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
    println!("Test completed successfully");
}
