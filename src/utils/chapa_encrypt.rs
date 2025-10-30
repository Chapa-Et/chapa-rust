//! The module to do the encryption for chapa requests as specified in
//! [here](https://developer.chapa.co/charge/encryption).

use crate::error::Result;

/// Encrypts the given data using the provided encryption key.
/// # Arguments[can be  changed to suit general usage]
/// * `data` - The data to be encrypted.
/// * `encryption_key` - The key to use for encryption.
/// # Returns
/// A `Result` containing the encrypted data as a `String` or an error.
pub fn encrypt_data(_data: &str, _encryption_key: &str) -> Result<String> {
    todo!();
    // with the current scope, implementing 3DES encryption is not feasible.
    // maybe in future releases.
}
