use num_bigint::{BigUint, ToBigUint};

fn clamp(key: &mut [u8]) {
    key[3] &= 15;
    key[7] &= 15;
    key[11] &= 15;
    key[15] &= 15;
    key[4] &= 252;
    key[8] &= 252;
    key[12] &= 252;
}

pub fn mac(msg: &[u8], key: &mut [u8]) -> anyhow::Result<Vec<u8>> {
    let zero = 0.to_biguint().unwrap();
    let two = 2.to_biguint().unwrap();
    let five = 5.to_biguint().unwrap();

    let r = &mut key[..16];
    clamp(r);
    let r = BigUint::from_bytes_le(r);

    let s = &key[16..];
    let s = BigUint::from_bytes_le(s);

    let mut acc: BigUint = zero;
    let p: BigUint = two.pow(130) - five;

    for i in 0..((msg.len() / 16) + 1) {
        let l = std::cmp::min(msg.len() - 16 * i, 16);
        let n: BigUint = two.pow(8*l as u32) + BigUint::from_bytes_le(&msg[16*i..(16*i+l)]);
        acc += n;
        acc = (acc * r.clone()) % p.clone();
    }

    acc = (acc + s) % two.pow(128);

    Ok(acc.to_bytes_le())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac() -> anyhow::Result<()> {
        let msg = vec![
            0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x67, 0x72,
            0x61, 0x70, 0x68, 0x69, 0x63, 0x20, 0x46, 0x6f,
            0x72, 0x75, 0x6d, 0x20, 0x52, 0x65, 0x73, 0x65,
            0x61, 0x72, 0x63, 0x68, 0x20, 0x47, 0x72, 0x6f,
            0x75, 0x70,
        ];

        let mut key = vec![
            0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33,
            0x7f, 0x44, 0x52, 0xfe, 0x42, 0xd5, 0x06, 0xa8,
            0x01, 0x03, 0x80, 0x8a, 0xfb, 0x0d, 0xb2, 0xfd,
            0x4a, 0xbf, 0xf6, 0xaf, 0x41, 0x49, 0xf5, 0x1b,
        ];

        let tag = mac(&msg, &mut key);

        assert_eq!(key[..16], vec![
            0x85, 0xd6, 0xbe, 0x08, 0x54, 0x55, 0x6d, 0x03,
            0x7c, 0x44, 0x52, 0x0e, 0x40, 0xd5, 0x06, 0x08,
        ]);
        
        assert_eq!(tag?, vec![
            0xa8, 0x06, 0x1d, 0xc1, 0x30, 0x51, 0x36, 0xc6,
            0xc2, 0x2b, 0x8b, 0xaf, 0x0c, 0x01, 0x27, 0xa9,
        ]);

        Ok(())
    }
}
