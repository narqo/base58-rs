const ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Refer to
/// https://tools.ietf.org/id/draft-msporny-base58-01.html
/// https://github.com/bitcoin/bitcoin/blob/v22.0/src/base58.cpp
pub fn base58_encode(input: &[u8]) -> String {
    let size = input.len();

    // ceil(log(256)/log(58)) simplified for integers
    let size = size * 138 / 100 + 1;

    let mut buf = vec![0u8; size];
    let mut length = 0;
    for b in input {
        let mut carry = *b as u32;
        let mut i = 0;
        for it in buf.iter_mut().rev() {
            if carry == 0 && i >= length {
                break;
            }
            i += 1;

            carry += 256 * (*it as u32);
            *it = (carry % 58) as u8;
            carry /= 58;
        }

        length = i;
    }

    let zlead = input.iter().take_while(|b| **b == 0).count();
    let skip = buf.iter().take_while(|b| **b == 0).count() - zlead;

    let mut res = String::with_capacity(buf.len() - skip);
    for b in &buf[skip..] {
        res.push(ALPHABET[(*b) as usize] as char);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_strings() {
        assert_eq!(base58_encode(b""), "");
        assert_eq!(base58_encode(b"ABC"), "NvLz");
        assert_eq!(base58_encode(b"Hello World!"), "2NEpo7TZRRrLZSi2U");
        assert_eq!(
            base58_encode(b"The quick brown fox jumps over the lazy dog."),
            "USm3fpXnKG5EUBx2ndxBDMPVciP5hGey2Jh4NDv6gmeo1LkMeiKrLJUUBk6Z"
        );
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(base58_encode(&[0x00, 0x00, 0x28, 0x7f, 0xb4, 0xcd]), "11233QC4");
    }
}
