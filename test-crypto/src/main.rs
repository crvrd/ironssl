// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate crypto;
extern crate rand;
extern crate cryptoimpl;

use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

use rand::Rng;

use std::fs::File;
use std::path::Path;
use std::io::Write;

// Encrypt a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);

    // Each encryption operation encrypts some data from
    // an input buffer into an output buffer. Those buffers
    // must be instances of RefReaderBuffer and RefWriteBuffer
    // (respectively) which keep track of how much data has been
    // read from or written to them.
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    // Each encryption operation will "make progress". "Making progress"
    // is a bit loosely defined, but basically, at the end of each operation
    // either BufferUnderflow or BufferOverflow will be returned (unless
    // there was an error). If the return value is BufferUnderflow, it means
    // that the operation ended while wanting more input data. If the return
    // value is BufferOverflow, it means that the operation ended because it
    // needed more space to output data. As long as the next call to the encryption
    // operation provides the space that was requested (either more input data
    // or more output space), the operation is guaranteed to get closer to
    // completing the full operation - ie: "make progress".
    //
    // Here, we pass the data to encrypt to the enryptor along with a fixed-size
    // output buffer. The 'true' flag indicates that the end of the data that
    // is to be encrypted is included in the input buffer (which is true, since
    // the input data includes all the data to encrypt). After each call, we copy
    // any output data to our result Vec. If we get a BufferOverflow, we keep
    // going in the loop since it means that there is more work to do. We can
    // complete as soon as we get a BufferUnderflow since the encryptor is telling
    // us that it stopped processing data due to not having any more data in the
    // input buffer.
    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
//
// This function is very similar to encrypt(), so, please reference
// comments in that function. In non-example code, if desired, it is possible to
// share much of the implementation using closures to hide the operation
// being performed. However, such code would make this example less clear.
fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

fn main() {
    let message = "Cry Havoc, and let slip the dogs of war!";

    let mut key: [u8; 32] = [0; 32];
    let mut test: [u8; 16] = [0xF0, 0xDD, 0xC8, 0x54, 0xB7, 0x6B, 0x67, 0xB3, 0x87, 0x38, 0xE6, 0x5, 0x36, 0x82, 0xD9, 0xA2];
    let mut iv: [u8; 16] = [0xdb, 0xf2, 0x01, 0xd4, 
                            0x13, 0x0a, 0x01, 0xd4, 
                            0x53, 0x22, 0x01, 0xd4, 
                            0x45, 0x5c, 0x01, 0xd5];
    let mut tmp: [u8; 16] = [0; 16];

    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut key);
    //rng.fill_bytes(&mut iv);
    //rng.fill_bytes(&mut test);

    cryptoimpl::aes::run_tests(&mut iv);

    for i in 0..16 {
        tmp[i] = iv[i];
        print!("0x{:X}\t", tmp[i]);
    }
    println!("\n");

    cryptoimpl::aes::encrypt_block(&mut iv, &mut test);

    cryptoimpl::aes::decrypt_block(&mut iv, &mut test);

    for i in 0..16 {
        print!("0x{:X}\t", iv[i]);
    }
    println!("\n");


    let encrypted_data = encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
    let decrypted_data = decrypt(&encrypted_data[..], &key, &iv).ok().unwrap();

    let epath = Path::new("encrypted.txt");

    let mut file = match File::create(&epath) {
        Err(why) => panic!("couldn't create file {}: {}", epath.display(), why),
        Ok(stuff) => stuff,
    };

    match file.write_all(&encrypted_data) {
        Err(why) => panic!("couldn't write to {}: {}", epath.display(), why),
        Ok(_) => println!("successfully wrote to {}", epath.display()),
    }

    let dpath = Path::new("decrypted.txt");

    let mut file = match File::create(&dpath) {
        Err(why) => panic!("couldn't create file {}: {}", dpath.display(), why),
        Ok(file) => file,
    };

    match file.write_all(&decrypted_data) {
        Err(why) => panic!("couldn't write to {}: {}", dpath.display(), why),
        Ok(_) => println!("successfully wrote to {}", dpath.display()),
    }

    assert!(message.as_bytes() == &decrypted_data[..]);
}
