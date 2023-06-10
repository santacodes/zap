use hex_literal::hex;
use sha2::{Sha256, Digest};

pub fn genhash(email: &str) -> &str {
    
    println!("Hashing Email {}",email);
    let mut hasher = Sha256::new();
    hasher.update(format!("{}",email));

    // read hash digest and consume hasher
    let result = hasher.finalize();
    println!("This is the result of hashing {:x}",result);
    assert_eq!(result[..], hex!("
    b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
    ")[..]);
    let output = stringify!(result);
    println!("This is the output {}",output);
    return output;
}
