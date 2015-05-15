use aesconst;

fn sub_bytes(data: &mut [u8; 16]) {
    for i in (0..data.len()) {
        data[i] = aesconst::SBOX[data[i] as usize];
    }
}

fn inv_sub_bytes(data: &mut [u8; 16]) {
    for i in (0..data.len()) {
        data[i] = aesconst::ISBOX[data[i] as usize];
    }
}

fn shift_rows(data: &mut [u8; 16]) {
    // AES uses column-major ordering
    let tmp: [u8; 16] =  
    [
        data[0],  data[5],  data[10],  data[15],
        data[4],  data[9],  data[14],  data[3],
        data[8], data[13], data[2],  data[7],
        data[12], data[1], data[6], data[11]
    ];

    for i in 0..data.len() {
        data[i] = tmp[i];
    }
}

fn inv_shift_rows(data: &mut [u8; 16]) {
    let tmp: [u8; 16] =  
    [
        data[0],  data[13],  data[10],  data[7],
        data[4],  data[1],  data[14],  data[11],
        data[8], data[5], data[2],  data[15],
        data[12], data[9], data[6], data[3]
    ];

    for i in 0..data.len() {
        data[i] = tmp[i];
    }
}

fn mix_cols(data: &mut [u8; 16]) {
    let mut tmp: [usize; 16] = [0; 16];
    for i in 0..16 {
        tmp[i] = data[i] as usize;
    }

    for i in 0..16 {
        let row = i % 4;
        data[i] = match row {
            0 => aesconst::MUL2[tmp[i]] ^ aesconst::MUL3[tmp[i+1]] ^ tmp[i+2] as u8           ^ tmp[i+3] as u8,
            1 => tmp[i-1] as u8         ^ aesconst::MUL2[tmp[i]]   ^ aesconst::MUL3[tmp[i+1]] ^ tmp[i+2] as u8,
            2 => tmp[i-2] as u8         ^ tmp[i-1] as u8           ^ aesconst::MUL2[tmp[i]]   ^ aesconst::MUL3[tmp[i+1]],
            3 => aesconst::MUL3[tmp[i-3]] ^ tmp[i-2] as u8         ^ tmp[i-1] as u8           ^ aesconst::MUL2[tmp[i]],
            _ => panic!("AES: Bad row when mixing columns: {}", row),
        };
    }

}

fn inv_mix_cols(data: &mut [u8; 16]) {
    let mut tmp: [usize; 16] = [0; 16];
    for i in 0..16 {
        tmp[i] = data[i] as usize;
    }

    for i in 0..16 {
        let row = i % 4;
        data[i] = match row {
            0 => aesconst::MUL14[tmp[i]]   ^ aesconst::MUL11[tmp[i+1]] ^ aesconst::MUL13[tmp[i+2]] ^ aesconst::MUL9[tmp[i+3]],
            1 => aesconst::MUL9[tmp[i-1]]  ^ aesconst::MUL14[tmp[i]]   ^ aesconst::MUL11[tmp[i+1]] ^ aesconst::MUL13[tmp[i+2]],
            2 => aesconst::MUL13[tmp[i-2]] ^ aesconst::MUL9[tmp[i-1]]  ^ aesconst::MUL14[tmp[i]]   ^ aesconst::MUL11[tmp[i+1]],
            3 => aesconst::MUL11[tmp[i-3]] ^ aesconst::MUL13[tmp[i-2]] ^ aesconst::MUL9[tmp[i-1]]  ^ aesconst::MUL14[tmp[i]],
            _ => panic!("AES: Bad row when inverse mixing columns: {}", row),
        };
    }

}

fn add_round_key(data: &mut[u8; 16], key: &[u8; 16]) {
    for i in 0..data.len() {
        data[i] = data[i] ^ key[i];
    }
}

fn schedule_keys(key: & [u8; 16]) -> [[u8; 16]; 11] {
    let mut roundkeys: [[u8; 16]; 11] = [[0; 16]; 11];
    let keys = 11;
    for i in 0..16 {
        roundkeys[0][i] = key[i];
    }

    for i in 1..keys {
        let mut t: [u8; 4] = [0; 4];
        t[0] = roundkeys[i-1][12];
        t[1] = roundkeys[i-1][13];
        t[2] = roundkeys[i-1][14];
        t[3] = roundkeys[i-1][15];

        roundkeys[i][0] = aesconst::SBOX[t[1] as usize] ^ aesconst::RCON[i-1];
        roundkeys[i][1] = aesconst::SBOX[t[2] as usize];
        roundkeys[i][2] = aesconst::SBOX[t[3] as usize];
        roundkeys[i][3] = aesconst::SBOX[t[0] as usize];

        for rd in 0..4 {
            roundkeys[i][rd] ^= roundkeys[i-1][rd];
        }

        for rd in 4..16 {
            roundkeys[i][rd] = roundkeys[i][rd-4] ^ roundkeys[i-1][rd];
        }
    }

    roundkeys
}

pub fn encrypt_block(data: &mut[u8; 16], roundkeys: &[[u8; 16]; 11]) {
    add_round_key(data, &roundkeys[0]);

    for i in 1..10 {
        sub_bytes(data);
        shift_rows(data);
        mix_cols(data);
        add_round_key(data, &roundkeys[i]);
    }

    sub_bytes(data);
    shift_rows(data);
    add_round_key(data, &roundkeys[roundkeys.len()-1]);
}

pub fn decrypt_block(data: &mut[u8; 16], roundkeys: &[[u8; 16]; 11]) {
    add_round_key(data, &roundkeys[roundkeys.len()-1]);

    for i in 1..10 {
        inv_shift_rows(data);
        inv_sub_bytes(data);
        add_round_key(data, &roundkeys[roundkeys.len()-i-1]);
        inv_mix_cols(data);
    }
    inv_shift_rows(data);
    inv_sub_bytes(data);
    add_round_key(data, &roundkeys[0]);
}


pub fn pkcs_pad(data: &mut Vec<u8>) {
    let bytes: usize = 16 - data.len()%16;
    let newlen = data.len() + bytes;
    for _ in data.len()..newlen {
        data.push(bytes as u8);
    }
}

pub fn pkcs_unpad(data: &mut Vec<u8>) {
    let removenum = data[data.len()-1];
    for _ in 0..removenum {
        data.pop();
    }
}

pub fn cbc_encrypt(data: &[u8], k: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut vdata: Vec<u8> = Vec::new();
    let mut finaldata: Vec<u8> = Vec::new();
    let mut blockdata: [u8; 16] = [0; 16];
    let mut key: [u8; 16] = [0; 16];
    let mut old: [u8; 16] = [0; 16];

    for i in 0..data.len() {
        vdata.push(data[i]);
    }
    for i in 0..16 {
        key[i] = k[i];
        old[i] = iv[i];
    }

    pkcs_pad(&mut vdata);

    let roundkeys: [[u8; 16]; 11] = schedule_keys(&key);

    for i in 0..(vdata.len()/16) {
        for j in 0..16 {
            blockdata[j] = vdata[i*16 + j] ^ old[j];
        }
        encrypt_block(&mut blockdata, &roundkeys);
        for j in 0..16 {
            finaldata.push(blockdata[j]);
            old[j] = blockdata[j];
        }
    }
    finaldata
}

pub fn cbc_decrypt(data: &[u8], k: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut vdata: Vec<u8> = Vec::new();
    let mut finaldata: Vec<u8> = Vec::new();
    let mut blockdata: [u8; 16] = [0; 16];
    let mut tmpdata: [u8; 16] = [0; 16];
    let mut key: [u8; 16] = [0; 16];
    let mut old: [u8; 16] = [0; 16];

    for i in 0..data.len() {
        vdata.push(data[i]);
    }
    for i in 0..16 {
        key[i] = k[i];
        old[i] = iv[i];
    }

    let roundkeys: [[u8; 16]; 11] = schedule_keys(&key);

    for i in 0..(vdata.len()/16) {
        for j in 0..16 {
            blockdata[j] = vdata[i*16 + j];
            tmpdata[j] = blockdata[j];
        }
        decrypt_block(&mut blockdata, &roundkeys);
        for j in 0..16 {
            finaldata.push(blockdata[j] ^ old[j]);
            old[j] = tmpdata[j];
        }
    }

    pkcs_unpad(&mut finaldata);

    finaldata
}

pub fn ecb_encrypt(data: &[u8], k: &[u8]) -> Vec<u8> {
    let mut vdata: Vec<u8> = Vec::new();
    let mut finaldata: Vec<u8> = Vec::new();
    let mut blockdata: [u8; 16] = [0; 16];
    let mut key: [u8; 16] = [0; 16];

    for i in 0..data.len() {
        vdata.push(data[i]);
    }
    for i in 0..16 {
        key[i] = k[i];
    }

    pkcs_pad(&mut vdata);

    let roundkeys: [[u8; 16]; 11] = schedule_keys(&key);

    for i in 0..(vdata.len()/16) {
        for j in 0..16 {
            blockdata[j] = vdata[i*16 + j];
        }
        encrypt_block(&mut blockdata, &roundkeys);
        for j in 0..16 {
            finaldata.push(blockdata[j]);
        }
    }
    finaldata
}

pub fn ecb_decrypt(data: &[u8], k: &[u8]) -> Vec<u8> {
    let mut vdata: Vec<u8> = Vec::new();
    let mut finaldata: Vec<u8> = Vec::new();
    let mut blockdata: [u8; 16] = [0; 16];
    let mut key: [u8; 16] = [0; 16];

    for i in 0..data.len() {
        vdata.push(data[i]);
    }
    for i in 0..16 {
        key[i] = k[i];
    }

    let roundkeys: [[u8; 16]; 11] = schedule_keys(&key);

    for i in 0..(vdata.len()/16) {
        for j in 0..16 {
            blockdata[j] = vdata[i*16 + j];
        }
        decrypt_block(&mut blockdata, &roundkeys);
        for j in 0..16 {
            finaldata.push(blockdata[j]);
        }
    }

    pkcs_unpad(&mut finaldata);

    finaldata
}