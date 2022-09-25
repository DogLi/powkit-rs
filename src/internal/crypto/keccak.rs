use sha3::{Digest, Keccak256, Keccak512};

pub fn keccak256(b: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::default();
    hasher.update(b);
    let out = hasher.finalize();
    out.iter().copied().collect()
}

pub fn keccak512(b: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak512::default();
    hasher.update(b);
    let out = hasher.finalize();
    out.iter().copied().collect()
}

pub const RCK8: [u32; 22] = [
    0x00000001, 0x00008082, 0x0000808A, 0x80008000, 0x0000808B, 0x80000001, 0x80008081, 0x00008009,
    0x0000008A, 0x00000088, 0x80008009, 0x8000000A, 0x8000808B, 0x0000008B, 0x00008089, 0x00008003,
    0x00008002, 0x00000080, 0x0000800A, 0x8000000A, 0x80008081, 0x00008080,
];

pub fn rol(x: u32, s: usize) -> u32 {
    x << s | x >> (32 - s)
}

pub fn keccak_f800(state: &mut [u32; 25]) {
    let mut aba = state[0];
    let mut abe = state[1];
    let mut abi = state[2];
    let mut abo = state[3];
    let mut abu = state[4];
    let mut aga = state[5];
    let mut age = state[6];
    let mut agi = state[7];
    let mut ago = state[8];
    let mut agu = state[9];
    let mut aka = state[10];
    let mut ake = state[11];
    let mut aki = state[12];
    let mut ako = state[13];
    let mut aku = state[14];
    let mut ama = state[15];
    let mut ame = state[16];
    let mut ami = state[17];
    let mut amo = state[18];
    let mut amu = state[19];
    let mut asa = state[20];
    let mut ase = state[21];
    let mut asi = state[22];
    let mut aso = state[23];
    let mut asu = state[24];

    let mut eba;
    let mut ebe;
    let mut ebi;
    let mut ebo;
    let mut ebu;

    let mut eka;
    let mut eke;
    let mut eki;
    let mut eko;
    let mut eku;

    let mut ema;
    let mut eme;
    let mut emi;
    let mut emo;
    let mut emu;

    let mut esa;
    let mut ese;
    let mut esi;
    let mut eso;
    let mut esu;

    let mut round = 0;
    while round < 22 {
        let mut ba = aba ^ aga ^ aka ^ ama ^ asa;
        let mut be = abe ^ age ^ ake ^ ame ^ ase;
        let mut bi = abi ^ agi ^ aki ^ ami ^ asi;
        let mut bo = abo ^ ago ^ ako ^ amo ^ aso;
        let mut bu = abu ^ agu ^ aku ^ amu ^ asu;

        let mut da = bu ^ rol(be, 1);
        let mut de = ba ^ rol(bi, 1);
        let mut di = be ^ rol(bo, 1);
        let mut do_ = bi ^ rol(bu, 1);
        let mut du = bo ^ rol(ba, 1);

        ba = aba ^ da;
        be = rol(age ^ de, 12);
        bi = rol(aki ^ di, 11);
        bo = rol(amo ^ do_, 21);
        bu = rol(asu ^ du, 14);
        eba = ba ^ (!be & bi) ^ RCK8[round];
        ebe = be ^ (!bi & bo);
        ebi = bi ^ (!bo & bu);
        ebo = bo ^ (!bu & ba);
        ebu = bu ^ (!ba & be);

        ba = rol(abo ^ do_, 28);
        be = rol(agu ^ du, 20);
        bi = rol(aka ^ da, 3);
        bo = rol(ame ^ de, 13);
        bu = rol(asi ^ di, 29);
        let ega = ba ^ (!be & bi);
        let ege = be ^ (!bi & bo);
        let egi = bi ^ (!bo & bu);
        let ego = bo ^ (!bu & ba);
        let egu = bu ^ (!ba & be);

        ba = rol(abe ^ de, 1);
        be = rol(agi ^ di, 6);
        bi = rol(ako ^ do_, 25);
        bo = rol(amu ^ du, 8);
        bu = rol(asa ^ da, 18);
        eka = ba ^ (!be & bi);
        eke = be ^ (!bi & bo);
        eki = bi ^ (!bo & bu);
        eko = bo ^ (!bu & ba);
        eku = bu ^ (!ba & be);

        ba = rol(abu ^ du, 27);
        be = rol(aga ^ da, 4);
        bi = rol(ake ^ de, 10);
        bo = rol(ami ^ di, 15);
        bu = rol(aso ^ do_, 24);
        ema = ba ^ (!be & bi);
        eme = be ^ (!bi & bo);
        emi = bi ^ (!bo & bu);
        emo = bo ^ (!bu & ba);
        emu = bu ^ (!ba & be);

        ba = rol(abi ^ di, 30);
        be = rol(ago ^ do_, 23);
        bi = rol(aku ^ du, 7);
        bo = rol(ama ^ da, 9);
        bu = rol(ase ^ de, 2);
        esa = ba ^ (!be & bi);
        ese = be ^ (!bi & bo);
        esi = bi ^ (!bo & bu);
        eso = bo ^ (!bu & ba);
        esu = bu ^ (!ba & be);

        /* round (round + 1): exx -> axx */

        ba = eba ^ ega ^ eka ^ ema ^ esa;
        be = ebe ^ ege ^ eke ^ eme ^ ese;
        bi = ebi ^ egi ^ eki ^ emi ^ esi;
        bo = ebo ^ ego ^ eko ^ emo ^ eso;
        bu = ebu ^ egu ^ eku ^ emu ^ esu;

        da = bu ^ rol(be, 1);
        de = ba ^ rol(bi, 1);
        di = be ^ rol(bo, 1);
        do_ = bi ^ rol(bu, 1);
        du = bo ^ rol(ba, 1);

        ba = eba ^ da;
        be = rol(ege ^ de, 12);
        bi = rol(eki ^ di, 11);
        bo = rol(emo ^ do_, 21);
        bu = rol(esu ^ du, 14);
        aba = ba ^ (!be & bi) ^ RCK8[round + 1];
        abe = be ^ (!bi & bo);
        abi = bi ^ (!bo & bu);
        abo = bo ^ (!bu & ba);
        abu = bu ^ (!ba & be);

        ba = rol(ebo ^ do_, 28);
        be = rol(egu ^ du, 20);
        bi = rol(eka ^ da, 3);
        bo = rol(eme ^ de, 13);
        bu = rol(esi ^ di, 29);
        aga = ba ^ (!be & bi);
        age = be ^ (!bi & bo);
        agi = bi ^ (!bo & bu);
        ago = bo ^ (!bu & ba);
        agu = bu ^ (!ba & be);

        ba = rol(ebe ^ de, 1);
        be = rol(egi ^ di, 6);
        bi = rol(eko ^ do_, 25);
        bo = rol(emu ^ du, 8);
        bu = rol(esa ^ da, 18);
        aka = ba ^ (!be & bi);
        ake = be ^ (!bi & bo);
        aki = bi ^ (!bo & bu);
        ako = bo ^ (!bu & ba);
        aku = bu ^ (!ba & be);

        ba = rol(ebu ^ du, 27);
        be = rol(ega ^ da, 4);
        bi = rol(eke ^ de, 10);
        bo = rol(emi ^ di, 15);
        bu = rol(eso ^ do_, 24);
        ama = ba ^ (!be & bi);
        ame = be ^ (!bi & bo);
        ami = bi ^ (!bo & bu);
        amo = bo ^ (!bu & ba);
        amu = bu ^ (!ba & be);

        ba = rol(ebi ^ di, 30);
        be = rol(ego ^ do_, 23);
        bi = rol(eku ^ du, 7);
        bo = rol(ema ^ da, 9);
        bu = rol(ese ^ de, 2);
        asa = ba ^ (!be & bi);
        ase = be ^ (!bi & bo);
        asi = bi ^ (!bo & bu);
        aso = bo ^ (!bu & ba);
        asu = bu ^ (!ba & be);

        round += 2;
    }

    state[0] = aba;
    state[1] = abe;
    state[2] = abi;
    state[3] = abo;
    state[4] = abu;
    state[5] = aga;
    state[6] = age;
    state[7] = agi;
    state[8] = ago;
    state[9] = agu;
    state[10] = aka;
    state[11] = ake;
    state[12] = aki;
    state[13] = ako;
    state[14] = aku;
    state[15] = ama;
    state[16] = ame;
    state[17] = ami;
    state[18] = amo;
    state[19] = amu;
    state[20] = asa;
    state[21] = ase;
    state[22] = asi;
    state[23] = aso;
    state[24] = asu;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_keccak_f800() {
        let mut state = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 65, 86, 69, 78, 67, 79, 73, 78, 75, 65, 87, 80, 79,
            87,
        ];
        keccak_f800(&mut state);
        let state_exp = [
            2727376398, 508243021, 2925876228, 3038525842, 779074219, 4021386812, 1973177222,
            1971903119, 150269505, 1978096212, 1043480230, 3070330841, 3343571286, 1787623575,
            85460266, 1901422822, 1249285963, 3359093104, 124051896, 1550870029, 3416720673,
            3924888459, 4003059341, 4262307665, 3596507164,
        ];
        assert_eq!(state, state_exp);
    }
}
