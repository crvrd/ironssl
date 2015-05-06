# rust-ecc
We need to rename this, since we're not doing ecc anymore.
We should also avoid rust-crypto, to avoid name conflicts with DaGenix &co.

Here's what I think we should do in order: //Edited by David after we talked about doing symmetric stuff
1. Learn Rust
2. Implement AES in rust, focus on CBC and CTR over anything else probably, and 256 bit over 128?
3. Implement a key-derivation function.  Idk what's easiest.  PBKDF2, bcrypt, scrypt?
4. Implement some other things maybe?
