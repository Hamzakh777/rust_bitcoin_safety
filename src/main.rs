use itertools::Itertools;
use ripemd::{Digest, Ripemd160};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha256::digest;
use std::{io, str::FromStr};
use std::time::Instant;

fn main() {
    let mut array: [usize; 16] = [0; 16];
    let mut array_start: usize = 0;
    let mut x = 6;
    let mut y = 10;
    let mut length = 13;

    println!("Array start number");
    array_start = read_to_number();

    println!("Please input x");
    x = read_to_number();

    println!("Please input y");
    y = read_to_number();

    let mut i: usize = 0;
    while i < 16 {
        array[i] = array_start + i;
        i += 1;
    }

    println!("Array A {:?}", array);

    // let now = Instant::now();
    while 
    // for combination in array.iter().permutations(length) {
    //     // let meow = combination.first().unwrap();
    //     if *combination[0] >= x
    //         && *combination[combination.len() - 1] <= y
    //         // && has_all_elements(&combination, &permutation)
    //     {
    //         println!("meow");
    //         // verify(&combination);
    //     }
    //     // let elapsed = now.elapsed();
    //     // println!("Elapsed: {:.2?}", elapsed);
    // }
    // let permutations = array.iter().permutations(9);
    // for permutation in permutations {
    // }
}

fn read_to_number() -> usize {
    let mut input = String::new();
    let mut res: usize = 0;

    if let Ok(_) = io::stdin().read_line(&mut input) {
        res = input.trim().parse::<usize>().unwrap();
    }

    res
}

fn verify(combination: &Vec<&&usize>) {
    // convert the combination numbers to binary (4 bits)
    let mut combination_in_bits: Vec<String> = vec![];
    for (i, value) in combination.iter().enumerate() {
        let bits = format!("{:b}", ***value);
        if bits.len() > 4 {
            let (_, second_slice) = bits.split_at(bits.len() - 4);
            combination_in_bits.push(second_slice.to_string());
        } else if combination.len() - 1 == i {
            combination_in_bits.push(bits.to_string());
        } else {
            combination_in_bits.push(format!("{:0>4}", bits.to_string()));
        }
    }

    let binary = combination_in_bits.join("");
    let hex = convert_binary_to_hex(&binary);
    let public_key = get_public_key_from_hex(&hex);
    let public_key_sha256 = public_key_to_sha256(&public_key.to_string());
    let ripemd160 = sha256_to_ripemd(&public_key_sha256);
}

fn convert_binary_to_hex(binary: &str) -> String {
    let base_10_number = u128::from_str_radix(&binary, 2).unwrap();

    format!("{:x}", base_10_number)
}

fn get_public_key_from_hex(hex: &str) -> PublicKey {
    let secp = Secp256k1::new();

    let formatted_hex = format!("{:0>64}", hex);

    let secret_key = SecretKey::from_str(&formatted_hex).unwrap();

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    public_key
}

fn public_key_to_sha256(public_key: &str) -> String {
    let hex = hex::decode(public_key).unwrap();
    digest(&hex[..])
}

fn sha256_to_ripemd(sha256: &str) -> String {
    let mut hasher = Ripemd160::new();
    let hex = hex::decode(sha256).unwrap();
    hasher.update(hex);
    let result = hasher.finalize();
    let resu = hex::encode(result);

    resu
}

fn has_all_elements(combination: &Vec<&usize>, original: &Vec<&usize>) -> bool {
    let original_set: std::collections::HashSet<&usize> = original.iter().cloned().collect();
    let combination_set: std::collections::HashSet<&usize> = combination.iter().cloned().collect();
    original_set.is_subset(&combination_set)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_all_elements() {
        let original: Vec<&usize> = vec![&1, &2, &3, &4, &5, &6, &7, &8, &9];
        let combination: Vec<&usize> = vec![&1, &2, &3, &4, &5, &6, &7, &9, &8, &4, &2, &4];

        assert_eq!(true, has_all_elements(&combination, &original))
    }

    #[test]
    fn test_verify() {
        let original = vec![&&1, &&2, &&3, &&15, &&16];

        verify(&original)
    }

    #[test]
    fn test_convert_binary_to_hex() {
        let value =
            "0111 0000 0001 0010 0011 0100 0101 0110 0111 1000 0001 0001 11 ".replace(" ", "");
        let result = convert_binary_to_hex(&value);

        assert_eq!(result, "1c048d159e047");
    }

    #[test]
    fn test_get_public_key_from_hex() {
        let private_key = "1c048d159e047";
        let public_key = get_public_key_from_hex(private_key);

        assert_eq!(
            public_key.to_string(),
            "02c81a115702c33f88b7bba091116b6ba7e9916267f26dd241351553c5a34a872f"
        )
    }

    #[test]
    fn test_public_key_to_sha256() {
        let public_key = "02c81a115702c33f88b7bba091116b6ba7e9916267f26dd241351553c5a34a872f";
        let public_key_sha256 = public_key_to_sha256(&public_key);

        assert_eq!(
            public_key_sha256,
            "7ceb4f18dfcdbdb3552b24d151efb5ba3e834680ff0bd108daa5c0d2ec50393f"
        );
    }

    #[test]
    fn test_sha256_to_ripemd160() {
        let sha256 = "7ceb4f18dfcdbdb3552b24d151efb5ba3e834680ff0bd108daa5c0d2ec50393f";

        let ripemd160 = sha256_to_ripemd(&sha256);
        assert_eq!(ripemd160, "24eb23f3cf0e14458f07ef0ce9d1e09c5e25e00d");
    }
}
