use aesconst;

pub fn test() {
    println!("test worked, you linked the mod correctly");
}

fn sub_bytes(data: &mut [u8; 16]) {
    for i in (0..data.len()) {
        data[i] = aesconst::SBOX[data[i] as usize];
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

fn mix_cols(data: &mut [u8; 16]) {
    for i in 0..16 {
        let row = i / 4;
        let col: [u8; 4] = match row {
            0 => [ data[i],    data[i+4], data[i+8], data[i+12] ],
            1 => [ data[i-4],  data[i],   data[i+4], data[i+8]  ],
            2 => [ data[i-8],  data[i-4], data[i],   data[i+4]  ],
            3 => [ data[i-12], data[i-8], data[i-4], data[i]    ],
            _ => panic!("Bad row when mixing columns: {}", row),
        };
        data[i] = match row {
            0 => aesconst::MUL2[col[0] as usize] ^ aesconst::MUL3[col[1] as usize] ^ col[2] ^ col[3],
            1 => col[0] ^ aesconst::MUL2[col[1] as usize] ^ aesconst::MUL3[col[2] as usize] ^ col[3],
            2 => col[0] ^ col[1] ^ aesconst::MUL2[col[2] as usize] ^ aesconst::MUL3[col[3] as usize],
            3 => aesconst::MUL3[col[0] as usize] ^ col[1] ^ col[2] ^ aesconst::MUL2[col[3] as usize],
            _ => panic!("Bad row when mixing columns: {}", row),
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
    add_round_key(data, &roundkeys[10]);
}