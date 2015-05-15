use aesconst;

pub fn test() {
    println!("test worked, you linked the mod correctly");
}

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
    let tmp: [u8; 16] =  
    [
        data[0],  data[1],  data[2],  data[3],
        data[5],  data[6],  data[7],  data[4],
        data[10], data[11], data[8],  data[9],
        data[15], data[12], data[13], data[14]
    ];

    for i in 0..data.len() {
        data[i] = tmp[i];
    }
}

fn inv_shift_rows(data: &mut [u8; 16]) {
    let tmp: [u8; 16] =  
    [
        data[0],  data[1],  data[2],  data[3],
        data[7],  data[4],  data[5],  data[6],
        data[10], data[11], data[8],  data[9],
        data[13], data[14], data[15], data[12]
    ];

    for i in 0..data.len() {
        data[i] = tmp[i];
    }
}

fn mix_cols(data: &mut [u8; 16]) {
    let mut tmp: [u8; 16] = [0; 16];
    for i in 0..16 {
        tmp[i] = data[i];
    }

    for i in 0..16 {
        let row = i / 4;
        let col: [u8; 4] = match row {
            0 => [ tmp[i],    tmp[i+4], tmp[i+8], tmp[i+12] ],
            1 => [ tmp[i-4],  tmp[i],   tmp[i+4], tmp[i+8]  ],
            2 => [ tmp[i-8],  tmp[i-4], tmp[i],   tmp[i+4]  ],
            3 => [ tmp[i-12], tmp[i-8], tmp[i-4], tmp[i]    ],
            _ => panic!("AES: Bad row when mixing columns: {}", row),
        };
        data[i] = match row {
            0 => aesconst::MUL2[col[0] as usize] ^ aesconst::MUL3[col[1] as usize] ^ col[2]                          ^ col[3],
            1 => col[0]                          ^ aesconst::MUL2[col[1] as usize] ^ aesconst::MUL3[col[2] as usize] ^ col[3],
            2 => col[0]                          ^ col[1]                          ^ aesconst::MUL2[col[2] as usize] ^ aesconst::MUL3[col[3] as usize],
            3 => aesconst::MUL3[col[0] as usize] ^ col[1]                          ^ col[2]                          ^ aesconst::MUL2[col[3] as usize],
            _ => panic!("AES: Bad row when mixing columns: {}", row),
        };
    }

}

fn inv_mix_cols(data: &mut [u8; 16]) {
    let mut tmp: [u8; 16] = [0; 16];
    for i in 0..16 {
        tmp[i] = data[i];
    }

    for i in 0..16 {
        let row = i / 4;
        let col: [u8; 4] = match row {
            0 => [ tmp[i],    tmp[i+4], tmp[i+8], tmp[i+12] ],
            1 => [ tmp[i-4],  tmp[i],   tmp[i+4], tmp[i+8]  ],
            2 => [ tmp[i-8],  tmp[i-4], tmp[i],   tmp[i+4]  ],
            3 => [ tmp[i-12], tmp[i-8], tmp[i-4], tmp[i]    ],
            _ => panic!("AES: Bad row when inverse mixing columns: {}", row),
        };
        data[i] = match row {
            0 => aesconst::MUL14[col[0] as usize] ^ aesconst::MUL11[col[1] as usize] ^ aesconst::MUL13[col[2] as usize] ^ aesconst::MUL9[col[3] as usize],
            1 => aesconst::MUL9[col[0] as usize]  ^ aesconst::MUL14[col[1] as usize] ^ aesconst::MUL11[col[2] as usize] ^ aesconst::MUL13[col[3] as usize],
            2 => aesconst::MUL13[col[0] as usize] ^ aesconst::MUL9[col[1] as usize]  ^ aesconst::MUL14[col[2] as usize] ^ aesconst::MUL11[col[3] as usize],
            3 => aesconst::MUL11[col[0] as usize] ^ aesconst::MUL13[col[1] as usize] ^ aesconst::MUL9[col[2] as usize]  ^ aesconst::MUL14[col[3] as usize],
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
        let mut t: [u8; 16] = [0; 16];
        t[0] = roundkeys[i-1][12];
        t[1] = roundkeys[i-1][13];
        t[2] = roundkeys[i-1][14];
        t[3] = roundkeys[i-1][15];

        roundkeys[i][0] = aesconst::SBOX[t[1] as usize];
        roundkeys[i][1] = aesconst::SBOX[t[2] as usize];
        roundkeys[i][2] = aesconst::SBOX[t[3] as usize];
        roundkeys[i][3] = aesconst::SBOX[t[0] as usize];

        roundkeys[i][0] ^= aesconst::RCON[i-1];

        for rd in 4..16 {
            roundkeys[i][rd] = roundkeys[i][rd-4] ^ roundkeys[i-1][rd];
        }
    }
    roundkeys
}

pub fn encrypt_block(data: &mut[u8; 16], key: &mut [u8; 16]) {
    let roundkeys: [[u8; 16]; 11] = schedule_keys(key);

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

pub fn decrypt_block(data: &mut[u8; 16], key: &mut [u8; 16]) {
    let roundkeys: [[u8; 16]; 11] = schedule_keys(key);

    add_round_key(data, &roundkeys[roundkeys.len()-1]);

    for i in 1..10 {
        inv_sub_bytes(data);
        inv_shift_rows(data);
        add_round_key(data, &roundkeys[roundkeys.len()-i-1]);
        inv_mix_cols(data);
    }

    inv_sub_bytes(data);
    inv_shift_rows(data);
    add_round_key(data, &roundkeys[0]);
}

fn print_data(data: &mut [u8; 16]) {
    for i in 0..data.len() {
        print!("0x{:X}\t", data[i]);
        if i%4 == 3 {
            println!("");
        }
    }
    println!("");
}

pub fn run_tests(data: &mut [u8; 16]) {
    test_mix_cols(data);
    test_sub_bytes(data);
    test_shift_rows(data);
    test_pkcs_pad();
}

pub fn test_mix_cols(data: &mut [u8; 16]) {
    println!("Testing Mix Columns:");
    
    print_data(data);

    mix_cols(data);

    print_data(data);

    inv_mix_cols(data);

    print_data(data);
}

pub fn test_sub_bytes(data: &mut [u8; 16]) {
    println!("Testing Sub Bytes:");
    
    print_data(data);

    sub_bytes(data);

    print_data(data);

    inv_sub_bytes(data);

    print_data(data);
}

pub fn test_shift_rows(data: &mut [u8; 16]) {
    println!("Testing Shift Rows:");
    
    print_data(data);

    shift_rows(data);

    print_data(data);

    inv_shift_rows(data);

    print_data(data);
}

pub fn pkcs_pad(data: &mut Vec<u8>) {
    let bytes: usize = 16 - data.len()%16;
    let newlen = data.len() + bytes;
    for i in data.len()..newlen {
        data.push(bytes as u8);
    }
}

pub fn pkcs_unpad(data: &mut Vec<u8>) {
    let removenum = data[data.len()-1];
    for i in 0..removenum {
        data.pop();
    }
}

pub fn test_pkcs_pad() {
    println!("Testing PKCS Padding:");


    let mut data = vec![0,6,7,8,4,23,4,3,6,1];

    for i in 0..data.len() {
        print!("0x{:X}\t", data[i]);
        if i%4 == 3 {
            println!("");
        }
    }
    println!("\n");

    pkcs_pad(&mut data);

    for i in 0..data.len() {
        print!("0x{:X}\t", data[i]);
        if i%4 == 3 {
            println!("");
        }
    }
    println!("\n");

    pkcs_unpad(&mut data);

    for i in 0..data.len() {
        print!("0x{:X}\t", data[i]);
        if i%4 == 3 {
            println!("");
        }
    }
    println!("\n");
}