// This functions makes use of the Caesar cipher algorithm to encrypt or decrypt a given text.
// For example: Encrypting 'C' with a shift of 3
// ASCII Value of 'C' = 67
// Base for uppercase letters = 65
// Position in alphabet = 67 - 65 = 2 (0-based index: 'A' = 0, 'B' = 1, 'C' = 2)
// Add shift: 2 + 3 = 5
// Handle wrapping (not needed in this example): (5 + 26) % 26 = 5
// Convert back to ASCII: 5 + 65 = 70 ('F')
// Therefore, 'C' is encrypted to 'F'

pub fn caesar_cipher(text: &str, shift: i32, encrypt: bool) -> String {
    // Map each character in the input string to a new character
    text.chars()
        .map(|c| {
            // Check if the character is an ASCII alphabetic character
            if c.is_ascii_alphabetic() {
                // Determine the ASCII value of the base character for the case
                let base = if c.is_ascii_uppercase() {
                    'A' as i32 // Base for uppercase letters
                } else {
                    'a' as i32 // Base for lowercase letters
                };

                let alphabet_size = 26;

                // Adjust the shift direction based on encryption or decryption
                let normalized_shift = if encrypt { shift } else { -shift };

                // Ensure the shift is positive and within the bounds of the alphabet
                let actual_shift =
                    ((normalized_shift % alphabet_size) + alphabet_size) % alphabet_size;

                // Shift the character and wrap it around the alphabet
                let shifted =
                    (c as i32 - base + actual_shift + alphabet_size) % alphabet_size + base;

                // Convert the shifted ASCII value back to a character
                char::from_u32(shifted as u32).unwrap()
            } else {
                // Return non-alphabetic characters unchanged
                c
            }
        })
        .collect() // Collect the mapped characters into a new String
}
