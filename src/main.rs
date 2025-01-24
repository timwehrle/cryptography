use caesar_cipher::caesar_cipher;

pub mod caesar_cipher;

fn main() {
    let message = "This is a test string!";

    let shift = 3;

    let encrypted = caesar_cipher(message, shift, true);
    println!("Encrypted: {}", encrypted);

    let decrypted = caesar_cipher(&encrypted, shift, false);
    println!("Decrypted: {}", decrypted);
}
