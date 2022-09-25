use byteorder::{ByteOrder, LittleEndian};

pub fn u32array_to_bytes(arr: &[u32]) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![0; arr.len() * 4];
    for i in 0..arr.len() {
        LittleEndian::write_u32(&mut buf[i * 4..], arr[i])
    }
    buf
}

pub fn bytes_to_u32array(buf: &[u8]) -> Vec<u32> {
    let len = buf.len() / 4;
    let mut arr: Vec<u32> = vec![0; len];
    for i in 0..len {
        arr[i] = LittleEndian::read_u32(&buf[i * 4..]);
    }
    arr
}

pub fn u32_to_bytes(val: u32) -> [u8; 4] {
    let mut data = [0; 4];
    LittleEndian::write_u32(&mut data, val);
    data
}

pub fn u64_to_bytes(val: u64) -> [u8; 8] {
    let mut data = [0; 8];
    LittleEndian::write_u64(&mut data, val);
    data
}
