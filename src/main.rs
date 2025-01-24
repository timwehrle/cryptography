pub mod ciphers;

use ciphers::caesar_cipher::caesar_cipher;
use ciphers::vigenere_cipher::vigenere_cipher;

fn test_cipher(
    name: &str,
    encrypt_fn: impl Fn(&str) -> String,
    decrypt_fn: impl Fn(&str) -> String,
) {
    let message = "This is a test string!";
    let encrypted = encrypt_fn(message);
    println!("Encrypted {}: {}", name, encrypted);

    let decrypted = decrypt_fn(&encrypted);
    println!("Decrypted {}: {}", name, decrypted);
}

fn main() {
    // Caesar Cipher Test
    let shift = 3;
    test_cipher(
        "Caesar Cipher",
        |msg| caesar_cipher(msg, shift, true),
        |msg| caesar_cipher(msg, shift, false),
    );

    // Vigenere Cipher Test
    let key = "hello";
    test_cipher(
        "Vigenere Cipher",
        |msg| vigenere_cipher(msg, key, true),
        |msg| vigenere_cipher(msg, key, false),
    );
}
