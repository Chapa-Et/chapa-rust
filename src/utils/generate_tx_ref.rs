//! Utility function to generate transaction references.
use rand::{Rng, distr::Alphanumeric};

/// Options for generating transaction references.
#[derive(Debug, Clone)]
pub struct GenTxRefOptions {
    /// Whether to remove the prefix from the generated transaction reference. default is false.
    remove_prefix: bool,
    /// The prefix to use for the generated transaction reference. default is "TX-".
    prefix: String,
    /// The size of the generated transaction reference. default is 15.
    size: usize,
    // /// The separator character to use between prefix and reference. default is '-'.
    // sep: char,
}

impl GenTxRefOptions {
    /// Creates a new `GenTxRefOptions` with the specified parameters.
    pub fn new(remove_prefix: Option<bool>, prefix: Option<String>, size: Option<usize>) -> Self {
        Self {
            remove_prefix: remove_prefix.unwrap_or(false),
            prefix: prefix.unwrap_or_else(|| "TX-".to_string()),
            size: size.unwrap_or(15),
        }
    }
}

impl Default for GenTxRefOptions {
    fn default() -> Self {
        Self {
            remove_prefix: false,
            prefix: "TX-".to_string(),
            size: 15,
        }
    }
}

/// Generates a transaction reference string based on the provided options.
/// # Arguments
/// * `options` - The options for generating the transaction reference.
/// # Returns
/// A generated transaction reference string.
pub fn generate_tx_ref(options: GenTxRefOptions) -> String {
    let generated = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(options.size)
        .map(char::from)
        .collect::<String>();
    if options.remove_prefix {
        generated
    } else {
        format!("{}{}", options.prefix, generated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tx_ref_default() {
        let options = GenTxRefOptions::default();
        let tx_ref = generate_tx_ref(options);
        assert!(tx_ref.starts_with("TX-"));
        assert_eq!(tx_ref.len(), 18); // "TX-" + 15 characters
    }

    #[test]
    fn test_generate_tx_ref_custom_options() {
        let options = GenTxRefOptions::new(None, Some("ORDER_".to_string()), Some(10));
        let tx_ref = generate_tx_ref(options);
        assert!(tx_ref.starts_with("ORDER_"));
        assert_eq!(tx_ref.len(), 16); // "ORDER_" + 10 characters
    }

    #[test]
    fn test_generate_tx_ref_remove_prefix() {
        let options = GenTxRefOptions::new(Some(true), None, Some(20));
        let tx_ref = generate_tx_ref(options);
        assert_eq!(tx_ref.len(), 20); // Only 20 characters, no prefix
    }
}
