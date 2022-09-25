
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Kiss99 {
    pub z: u32,
    pub w: u32,
    pub jsr: u32,
    pub jcong: u32,
}


impl Kiss99 {
    pub fn new(z: u32, w: u32, jsr: u32, jcong: u32) -> Self {
        Self { z, w, jsr, jcong }
    }
    pub fn kiss(&mut self) -> u32 {
        self.z = 36969 * (self.z & 65535) + (self.z >> 16);
        self.w = 18000 * (self.w & 65535) + (self.w >> 16);
        self.jcong = (69069 * self.jcong as u64 + 1234567) as u32;

        self.jsr = self.jsr ^ (self.jsr << 17);
        self.jsr = self.jsr ^ (self.jsr >> 13);
        self.jsr = self.jsr ^ (self.jsr << 5);

        ((((self.z << 16) as u64 + self.w as u64) as u32 ^ self.jcong) as u64 + self.jsr as u64)
            as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kiss99() {
        #[derive(Debug)]
        struct Config {
            z: u32,
            w: u32,
            jsr: u32,
            jcong: u32,
            index: usize,
            value: u32,
        }
        let cases = vec![
            Config {
                z: 362436069,
                w: 521288629,
                jsr: 123456789,
                jcong: 380116160,
                index: 1,
                value: 769445856,
            },
            Config {
                z: 362436069,
                w: 521288629,
                jsr: 123456789,
                jcong: 380116160,
                index: 2,
                value: 742012328,
            },
            Config {
                z: 362436069,
                w: 521288629,
                jsr: 123456789,
                jcong: 380116160,
                index: 3,
                value: 2121196314,
            },
            Config {
                z: 362436069,
                w: 521288629,
                jsr: 123456789,
                jcong: 380116160,
                index: 4,
                value: 2805620942,
            },
            Config {
                z: 362436069,
                w: 521288629,
                jsr: 123456789,
                jcong: 380116160,
                index: 100000,
                value: 941074834,
            },
        ];
        for tt in cases {
            let mut rng = Kiss99::new(tt.z, tt.w, tt.jsr, tt.jcong);
            let mut value = 0;
            for _i in 0..tt.index {
                value = rng.kiss();
            }
            assert_eq!(value, tt.value);
        }
    }
}
