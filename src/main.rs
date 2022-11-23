use cipher_crypt::Caesar;
use cipher_crypt::Cipher;
use std::collections::HashMap;

#[derive(Debug)]
struct vignere_cipher {
    table: Vec<Vec<char>>,
    letter_to_index: HashMap<char, usize>,
}

impl vignere_cipher {
    pub fn new() -> Self {
        let mut table = Vec::new();
        let mut letter_to_index = HashMap::new();
        let mut row = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        // table.push(row.clone());
        for i in 0..26 {
            table.push(row.clone());
            letter_to_index.insert(*row.get(0).unwrap(), i);
            row.rotate_left(1);
        }

        Self {
            table,
            letter_to_index,
        }
    }
}

fn main() {
    let keyword = "lemon";
    let plaintext = "attackatdawn";
    let keyword_length = keyword.chars().count();
    let plaintext_length = plaintext.chars().count();
    let mut keyword_fitted = keyword.clone();
    let mut times_to_repeat = 0;
    if keyword_length < plaintext_length {
        times_to_repeat = (plaintext_length / keyword_length) + 1;
    }
    let keyword_repeated = keyword_fitted.repeat(times_to_repeat);
    keyword_fitted = &keyword_repeated[0..(plaintext_length)];
    println!("{}", keyword_fitted);

    let vc = vignere_cipher::new();
    // println!("{:?}", vc);
    // println!("{:?}", vc.letter_to_column_index);

    // encrypt
    let mut ciphertext = String::new();
    for (i, keyword_char) in keyword_fitted.chars().enumerate() {
        let col_index = vc.letter_to_index.get(&keyword_char).unwrap();
        let plaintext_char = plaintext.chars().nth(i).unwrap();
        let row_index = vc.letter_to_index.get(&plaintext_char).unwrap();
        let cipher_char = vc.table.get(*row_index).unwrap().get(*col_index).unwrap();
        ciphertext.push(cipher_char.clone());
    }
    println!("ciphertext {:?}", ciphertext);

    //decrypt
    let mut plaintext_after_decrypt = String::new();
    for (i, keyword_char) in keyword_fitted.chars().enumerate() {
        println!("keyword char {:?}", keyword_char);
        let row_index = vc.letter_to_index.get(&keyword_char).unwrap();
        let ciphertext_char = ciphertext.chars().nth(i).unwrap();
        println!("ciphertext char {:?}", ciphertext_char);
        let keyword_row = vc.table.get(*row_index).unwrap();
        for j in 0..keyword_row.len() {
            let char_in_row = keyword_row.get(j).unwrap();
            if *char_in_row == ciphertext_char {
                // use the first row in table to reference original alphabet order as plaintext
                let plain_char = vc.table.get(0).unwrap().get(j).unwrap();
                plaintext_after_decrypt.push(plain_char.clone());
            }
        }
    }
    println!("plaintext {:?}", plaintext_after_decrypt);
}
