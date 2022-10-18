use powkit::kawpow::client::Client;
use std::path::PathBuf;

struct Test {
    height: u64,
    nonce: u64,
    hash: [u8; 32],
    mix: Vec<u8>,
    digest: Vec<u8>,
}

#[test]
fn test_compute_rvn_coin() {
    let tests = vec![
        Test {
            height: 0,
            nonce: 0x0000000000000000,
            hash: hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("6e97b47b134fda0c7888802988e1a373affeb28bcd813b6e9a0fc669c935d03a")
                .unwrap(),
            digest: hex::decode("e601a7257a70dc48fccc97a7330d704d776047623b92883d77111fb36870f3d1")
                .unwrap(),
        },
        Test {
            height: 49,
            nonce: 0x0000000007073c07,
            hash: hex::decode("63155f732f2bf556967f906155b510c917e48e99685ead76ea83f4eca03ab12b")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("d36f7e815ee09e74eceb9c96993a3d681edf2bf0921fc7bb710364042db99777")
                .unwrap(),
            digest: hex::decode("e7ced124598fd2500a55ad9f9f48e3569327fe50493c77a4ac9799b96efb9463")
                .unwrap(),
        },
        Test {
            height: 50,
            nonce: 0x00000000076e482e,
            hash: hex::decode("9e7248f20914913a73d80a70174c331b1d34f260535ac3631d770e656b5dd922")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("d6dc634ae837e2785b347648ea515e25e5d8821ae0b95e1c2a9c2d497e0dcfbd")
                .unwrap(),
            digest: hex::decode("ab0ad7ef8d8ee317dd12d10310aceed7321d34fb263791c2de5776a6658d177e")
                .unwrap(),
        },
        Test {
            height: 99,
            nonce: 0x000000003917afab,
            hash: hex::decode("de37e1824c86d35d154cf65a88de6d9286aec4f7f10c3fc9f0fa1bcc2687188d")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("fa706860e5e0e830d5d1d7157e5bea7f5f8a350c7c8612ac1d1fcf2974d64244")
                .unwrap(),
            digest: hex::decode("aa85340690f2e907054324a5021937910e15edfd1ef1577231843e7d32ec3a61")
                .unwrap(),
        },
        Test {
            height: 29950,
            nonce: 0x005d409dbc23a62a,
            hash: hex::decode("ac7b55e801511b77e11d52e9599206101550144525b5679f2dab19386f23dcce")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("5359807b77a74878269c3a3044df8618a576ce8dc52e1c48d927d4a60e7c6b79")
                .unwrap(),
            digest: hex::decode("022019e5408683f7f8326b4e46b42864a3a069f17b6151e434fcaedecaadd918")
                .unwrap(),
        },
        Test {
            height: 29999,
            nonce: 0x005db5fa4c2a3d03,
            hash: hex::decode("e43d7e0bdc8a4a3f6e291a5ed790b9fa1a0948a2b9e33c844888690847de19f5")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("d15de3f9bfedd9b6d0f498273eb3b437115bdc8326c96c6457ac06deb5c9f389")
                .unwrap(),
            digest: hex::decode("4e93630b81198752f876b24380999189b7b9366c08222ac05e4237b87114f305")
                .unwrap(),
        },
        Test {
            height: 30000,
            nonce: 0x005db8607994ff30,
            hash: hex::decode("d34519f72c97cae8892c277776259db3320820cb5279a299d0ef1e155e5c6454")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("de0348b69bf91dfe2c3d3dba6f0132e9048a5284e57b8d9d20adc5f3dc0d3236")
                .unwrap(),
            digest: hex::decode("c7953d848cda6e304f77b4c6d735645c8e8508a5e74c9e9814ef37b19087cd6c")
                .unwrap(),
        },
        Test {
            height: 30049,
            nonce: 0x005e2e215a8ca2e7,
            hash: hex::decode("8b6ce5da0b06d18db7bd8492d9e5717f8b53e7e098d9fef7886d58a6e913ef64")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("975c6a9decc89cba7ace69338d4de8510d9619aef42b1d35d0bef7e0ce0614a9")
                .unwrap(),
            digest: hex::decode("c262d8055e288d04b951a844bfca8ba529f5b4d652b408e3942727d7dd90957a")
                .unwrap(),
        },
        Test {
            height: 30050,
            nonce: 0x005e30899481055e,
            hash: hex::decode("c2c46173481b9ced61123d2e293b42ede5a1b323210eb2a684df0874ffe09047")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("362f2fabdb9699d3634b6499703f939f378ee4eac803396c2b0ed0fe1d154972")
                .unwrap(),
            digest: hex::decode("4cd7e6e79e0b63d42b2b06716a919ccc7834077ec727a9ea94edcdaff2fefab8")
                .unwrap(),
        },
        Test {
            height: 30099,
            nonce: 0x005ea6aef136f88b,
            hash: hex::decode("ea42197eb2ba79c63cb5e655b8b1f612c5f08aae1a49ff236795a3516d87bc71")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("b1196457261bd05ccb387a8ff3fd02687bf496bd7943d89419465289669e27aa")
                .unwrap(),
            digest: hex::decode("39d1ebfa783b61a6fa8e9747d0f9f134efae5cfba284a2c80e8deabae6b98676")
                .unwrap(),
        },
        Test {
            height: 59950,
            nonce: 0x02ebe0503bd7b1da,
            hash: hex::decode("49e15ba4bf501ce8fe8876101c808e24c69a859be15de554bf85dbc095491bd6")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("df3dbb1669fd35dbb0ae96bbea2d498f0c6992cbddd092aeace42dd933505f95")
                .unwrap(),
            digest: hex::decode("b8984cf4021c4433f753654848d721f33a0792b4417241f0cf7c7c2db011a54a")
                .unwrap(),
        },
        Test {
            height: 59999,
            nonce: 0x02edb6275bd221e3,
            hash: hex::decode("f5c50ba5c0d6210ddb16250ec3efda178de857b2b1703d8d5403bd0f848e19cf")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("5017df70e97ca35638cf439cdbe54f30383d335e18eb4a74d6e166736f1038fa")
                .unwrap(),
            digest: hex::decode("4cf1fa62f25b577ac822a6a28d55f8b7e3ae7fe983abd868ae00927e68c41016")
                .unwrap(),
        },
        Test {
            height: 170915,
            nonce: 0x0000000044975727,
            hash: hex::decode("5b3e8dfa1aafd3924a51f33e2d672d8dae32fa528d8b1d378d6e4db0ec5d665d")
                .unwrap()
                .try_into()
                .unwrap(),
            mix: hex::decode("efb29147484c434f1cc59629da90fd0343e3b047407ecd36e9ad973bd51bbac5")
                .unwrap(),
            digest: hex::decode("e7e6bb3b2f9acd3864bc86f72f87237eaf475633ef650c726ac80eb0adf116b6")
                .unwrap(),
        },
    ];
    let client = Client::new_raven_coin(PathBuf::from("/tmp"));
    for tt in tests {
        let (mix, digest) = client.compute(tt.hash, tt.height, tt.nonce).unwrap();
        assert_eq!(mix, tt.mix);
        assert_eq!(digest, tt.digest);
    }
}
