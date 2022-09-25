fn rotl32(a: u32, b: u32) -> u32 {
    let i = (32 - b as i64) as u32;
    a << (b & 31) | a >> (i & 31)
}

fn rotr32(a: u32, b: u32) -> u32 {
    let i = (32 - b as i64) as u32;
    a << (i & 31) | a >> (b & 31)
}

fn clz32(a: u32) -> u32 {
    a.leading_zeros()
}

fn popcount32(a: u32) -> u32 {
    a.count_ones()
}

fn mul_hi32(a: u32, b: u32) -> u32 {
    ((a as u64 * b as u64) >> 32) as u32
}

pub fn random_math(a: u32, b: u32, selector: u32) -> u32 {
    match selector % 11 {
        0 => (a as u64 + b as u64) as u32,
        1 => (a as u64 * b as u64) as u32,
        2 => mul_hi32(a, b),
        3 => a.min(b),
        4 => rotl32(a, b),
        5 => rotr32(a, b),
        6 => a & b,
        7 => a | b,
        8 => a ^ b,
        9 => (clz32(a) as u64 + clz32(b) as u64) as u32,
        10 => (popcount32(a) as u64 + popcount32(b) as u64) as u32,
        _ => 0,
    }
}

pub fn random_merge(a: u32, b: u32, selector: u32) -> u32 {
    let x = ((selector >> 16) % 31) + 1;
    match selector % 4 {
        0 => (a as u64 * 33 + b as u64) as u32,
        1 => ((a ^ b) as u64 * 33) as u32,
        2 => rotl32(a, x) ^ b,
        3 => rotr32(a, x) ^ b,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct Config {
        a: u32,
        b: u32,
        selector: u32,
        value: u32,
    }

    #[test]
    fn test_random_math() {
        let tests = vec![
            Config {
                a: 0x8626BB1F,
                b: 0xBBDFBC4E,
                selector: 0x883E5B49,
                value: 0x4206776D,
            },
            Config {
                a: 0x3F4BDFAC,
                b: 0xD79E414F,
                selector: 0x36B71236,
                value: 0x4C5CB214,
            },
            Config {
                a: 0x6D175B7E,
                b: 0xC4E89D4C,
                selector: 0x944ECABB,
                value: 0x53E9023F,
            },
            Config {
                a: 0x2EDDD94C,
                b: 0x7E70CB54,
                selector: 0x3F472A85,
                value: 0x2EDDD94C,
            },
            Config {
                a: 0x61AE0E62,
                b: 0xe0596b32,
                selector: 0x3F472A85,
                value: 0x61AE0E62,
            },
            Config {
                a: 0x8A81E396,
                b: 0x3F4BDFAC,
                selector: 0xCEC46E67,
                value: 0x1E3968A8,
            },
            Config {
                a: 0x8A81E396,
                b: 0x7E70CB54,
                selector: 0xDBE71FF7,
                value: 0x1E3968A8,
            },
            Config {
                a: 0xA7352F36,
                b: 0xA0EB7045,
                selector: 0x59E7B9D8,
                value: 0xA0212004,
            },
            Config {
                a: 0xC89805AF,
                b: 0x64291E2F,
                selector: 0x1BDC84A9,
                value: 0xECB91FAF,
            },
            Config {
                a: 0x760726D3,
                b: 0x79FC6A48,
                selector: 0xC675CAC5,
                value: 0x0FFB4C9B,
            },
            Config {
                a: 0x75551D43,
                b: 0x3383BA34,
                selector: 0x2863AD31,
                value: 0x00000003,
            },
            Config {
                a: 0xEA260841,
                b: 0xE92C44B7,
                selector: 0xF83FFE7D,
                value: 0x0000001B,
            },
        ];
        for tt in tests {
            let value = random_math(tt.a, tt.b, tt.selector);
            assert_eq!(value, tt.value);
        }
    }

    #[test]
    fn test_random_merge() {
        let tests = vec![
            Config {
                a: 0x3B0BB37D,
                b: 0xA0212004,
                selector: 0x9BD26AB0,
                value: 0x3CA34321,
            },
            Config {
                a: 0x10C02F0D,
                b: 0x870FA227,
                selector: 0xD4F45515,
                value: 0x91C1326A,
            },
            Config {
                a: 0x24D2BAE4,
                b: 0x0FFB4C9B,
                selector: 0x7FDBC2F2,
                value: 0x2EDDD94C,
            },
            Config {
                a: 0xDA39E821,
                b: 0x089C4008,
                selector: 0x8B6CD8C3,
                value: 0x8A81E396,
            },
        ];
        for tt in tests {
            let value = random_merge(tt.a, tt.b, tt.selector);
            assert_eq!(tt.value, value);
        }
    }
}
