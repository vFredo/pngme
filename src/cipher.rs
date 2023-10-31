/// Encode a message given in bytes using the key
/// and return the encoded bytes
pub fn xor_encode(data: &[u8], key: &str) -> Vec<u8> {
    if key.len() == 0 {
        return data.to_vec();
    }

    let key_bytes = key.as_bytes();
    data.iter()
        .zip(key_bytes.iter().cycle())
        .map(|(&d, &k)| d ^ k)
        .collect()
}

/// Decode a message given in bytes using the key
/// and return the message as a String
pub fn xor_decode(data: &[u8], key: &str) -> String {
    if key.len() == 0 {
        return String::from_utf8(data.to_vec()).unwrap();
    }

    let key_bytes = key.as_bytes();
    let result: Vec<u8> = data
        .iter()
        .zip(key_bytes.iter().cycle())
        .map(|(&d, &k)| d ^ k)
        .collect();

    String::from_utf8(result).unwrap()
}
