#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_encode(){
        let text = String::from("Many hands make light work.");
        assert_eq!(base64::base64::encode(&text.into_bytes()), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu")
    }
}
