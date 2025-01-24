// This function uses the Vigenere cipher algorithm to encrypt or decrypt a given text using a repeating key.
// The Vigenere cipher shifts each character in the input text by an amount determined by the corresponding character in the key.
// If `encrypt` is true, the function encrypts the input text; otherwise, it decrypts it.
// For example:
// Encrypting 'C' with a key character 'D'
// ASCII Value of 'C' = 67
// Base for uppercase letters = 65
// Position in alphabet = 67 - 65 = 2 (0-based index: 'A' = 0, 'B' = 1, 'C' = 2)
// Key shift from 'D': ASCII Value of 'D' = 68, Base = 65 -> Shift = 68 - 65 = 3
// Add shift for encryption: 2 + 3 = 5
// Handle wrapping: (5 + 26) % 26 = 5
// Convert back to ASCII: 5 + 65 = 70 ('F')
// Therefore, 'C' encrypted with key 'D' becomes 'F'.

pub fn vigenere_cipher(text: &str, key: &str, encrypt: bool) -> String {
    let mut result = String::new();
    let key_bytes = key.as_bytes();
    let key_len = key.len();
    let mut key_index = 0;

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            // Determine the base ASCII value based on the case of the character
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };

            // Get the corresponding key character
            let key_char = key_bytes[key_index];

            // Calculate the shift value from the key character
            // Normalize the key character to its alphabetic index
            let key_shift = if c.is_ascii_uppercase() {
                (key_char as u8 - b'A') % 26
            } else {
                (key_char as u8 - b'a') % 26
            };

            let shifted = if encrypt {
                // For encryption shift forward
                (c as u8 - base + key_shift) % 26 + base
            } else {
                // For decryption shift backward
                (c as u8 - base + 26 - key_shift) % 26 + base
            };

            // Convert the shifted value back to a character and append to the result
            result.push(shifted as char);

            // Move to next character in the key
            key_index = (key_index + 1) % key_len;
        } else {
            // Return non-alphabetic characters unchanged
            result.push(c);
        }
    }

    result
}
