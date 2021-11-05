pub mod base64 {
    static BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    fn get_b64_char(val: u8) -> char { 
        match BASE64_TABLE.chars().nth((val & 0x3F) as usize) { 
           None => '0', 
           Some(x) => x
        }
    }

    fn encode_chunk(chunk: &[u8]) -> [char; 4]{
        let mut tmp : [char; 4] = ['0'; 4];

        tmp[0] = get_b64_char(chunk[0] >> 2);
        tmp[1] = get_b64_char((chunk[0] << 4) | (chunk[1] >> 4));
        tmp[2] = get_b64_char((chunk[1] << 2) | (chunk[2] >> 6));
        tmp[3] = get_b64_char(chunk[2]);

        tmp
    }

    fn add_padding(chunk: &[u8]) -> [char; 4]{
        let mut tmp : [char; 4] = ['0'; 4];
        tmp[0] = get_b64_char(chunk[0] >> 2);

        match chunk.len() {
            1 => {
                    tmp[1] = get_b64_char((chunk[0] << 4) | 0x00); 
                    tmp[2] = '=';
                }
            2 => {
                    tmp[1] = get_b64_char((chunk[0] << 4) | (chunk[1] >> 4));
                    tmp[2] = get_b64_char((chunk[1] << 2) | (chunk[2] >> 6));
                }
            _ => panic!("Invalid padding!")
        };

        tmp[3] = '=';
        tmp

    }

    pub fn encode(data: &Vec<u8>) -> String {
        let mut res = String::new();
        let chunks = data.chunks_exact(3);
        let remainder = chunks.remainder(); // For loop will consume the iterator, so the remainder must be copied

        for chunk in chunks {
            res.extend(encode_chunk(chunk)); 
        }
        if !remainder.is_empty() { 
            res.extend(add_padding(remainder));
        }

        res
    }
}