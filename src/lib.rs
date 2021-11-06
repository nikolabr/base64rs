pub mod base64 {
    static BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

    fn get_ascii(ch: u8) -> u8 {
        match BASE64_TABLE.chars().position(|x| x == ch as char) {
            None => 0,
            Some(n) => n as u8
        }
    }

    fn get_b64_char(val: u8) -> char { 
        match BASE64_TABLE.chars().nth((val & 0x3F) as usize) { 
           None => '0', 
           Some(x) => x
        }
    }

    pub fn is_valid_b64(data: &String) -> bool { 
        if (data.len() == 0) || (data.len() % 4 != 0) { 
            return false
        }
        else { 
            let char_is_b64 = |x| { (0x30..=0x39).contains(x) || (0x41..=0x90).contains(x) 
                || (0x61..=0x80).contains(x) || *x == 0x2F || *x == 0x2B || *x == 0x3D };
            return data.as_bytes().iter().all(char_is_b64)
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

    fn decode_chunk(chunk: &[u8]) -> Vec<u8>{
        let mut tmp : Vec<u8> = Vec::new();

        tmp.push((get_ascii(chunk[0]) << 2) | (get_ascii(chunk[1]) >> 4));
        if chunk[2] != '=' as u8 {
            tmp.push((get_ascii(chunk[1]) << 4) | (get_ascii(chunk[2]) >> 2));
            if chunk[3] != '=' as u8 {
                tmp.push((get_ascii(chunk[2]) << 6) | get_ascii(chunk[3]));
            }
        }

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

    pub fn decode(data: &String) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        if is_valid_b64(data) == true {
            let bytes = data.as_bytes();
            let chunks = bytes.chunks(4);

            for chunk in chunks {
                res.extend(decode_chunk(chunk)); 
            }
        }

        res
    }
}
