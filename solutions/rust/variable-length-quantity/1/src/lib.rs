#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::<u8>::new();

    for value in values {
        let mut value_result = Vec::<u8>::new();

        let mut is_first_byte = true;
        let mask: u8 = 0x7F;
        for i in 0_u8..5 {
            let tmp = (value >> 7 * i) as u8 & mask;

            if is_first_byte {
                value_result.push(tmp);
                is_first_byte = false;
            } else {
                value_result.push(tmp | 0x80);
            }
        }
        value_result.reverse();

        while value_result[0] == 0x80 {
            value_result.remove(0);
        }

        result.append(&mut value_result);
    }

    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::<u32>::new();
    
    let mut value: u32 = 0;
    let mut need_more_bytes = false;
    
    for byte in bytes {
        value |= (byte & 0x7F) as u32;
        if byte & 0x80 != 0 {
            need_more_bytes = true;
            value <<= 7;
        } else {
            need_more_bytes = false;
            
            result.push(value);
            
            value = 0;
        }
    }
    
    if need_more_bytes {
        Err(Error::IncompleteNumber)
    } else {
        Ok(result)
    }
}
