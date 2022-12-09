pub mod rsa;

use rsa::Keypair;

fn main() {
    let keypair = Keypair::new(256);
    let cipher_text = keypair.pubkey().encrypt("Hello, world!");
    println!("{:?}", cipher_text);
    let original = keypair.decrypt(&cipher_text);
    println!("{:?}", String::from_utf8(original));
}
