/// Encode a message given in bytes using the key
/// and return the encoded bytes
pub fn xor_encode(data: &[u8], key: &str) -> Vec<u8> {
    let key_bytes = key.as_bytes();
    let mut result = Vec::new();

    for i in 0..data.len() {
        result.push(data[i] ^ key_bytes[i % key_bytes.len()])
    }

    result
}

/// Decode a message given in bytes using the key
/// and return the message as a String
pub fn xor_decode(data: &[u8], key: &str) -> String {
    let key_bytes = key.as_bytes();
    let mut result = Vec::new();

    for i in 0..data.len() {
        result.push(data[i] ^ key_bytes[i % key_bytes.len()])
    }

    String::from_utf8(result).unwrap()
}
