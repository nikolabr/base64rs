#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_encode(){
        let text = String::from("Many hands make light work.");
        assert_eq!(base64::base64::encode(&text.into_bytes()), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
        assert_eq!(base64::base64::encode(&String::from("").into_bytes()), "");
    }

    #[test] 
    fn test_valid_ascii(){ // Test an ascii string
        assert_eq!(base64::base64::is_valid_b64("    ".to_string()), false);
        assert_eq!(base64::base64::is_valid_b64("\r\r\r\r".to_string()), false);
        assert_eq!(base64::base64::is_valid_b64("\n\n\n\n".to_string()), false);
        assert_eq!(base64::base64::is_valid_b64("".to_string()), false);
        assert_eq!(base64::base64::is_valid_b64("\0".to_string()), false);
    }

    #[test] 
    fn test_valid_b64(){ // Test a base64 string
        assert_eq!(base64::base64::is_valid_b64("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".to_string()), true);
        assert_eq!(base64::base64::is_valid_b64("Y3J1c3RhY2Vhbg==".to_string()), true);
    }

    #[test]
    fn test_valid_utf8(){ // Test unicode
        assert_eq!(base64::base64::is_valid_b64("Россия".to_string()), false);   
        assert_eq!(base64::base64::is_valid_b64("日本国".to_string()), false);   
    }
}
