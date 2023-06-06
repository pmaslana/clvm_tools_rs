use bls12_381::hash_to_curve::{ExpandMsgXmd, HashToCurve};
use bls12_381::{G1Affine, G1Projective};

use crate::classic::clvm::__type_compatibility__::{Bytes, BytesFromType};
use crate::tests::classic::run::do_basic_run;

#[test]
fn test_using_bls_operators_0() {
    // Run a program which uses the map_to_g1 operator and check the output.
    let msg: &[u8] = &[
        0x97, 0x90, 0x63, 0x5d, 0xe8, 0x74, 0x0e, 0x9a, 0x6a, 0x6b, 0x15, 0xfb, 0x6b, 0x72, 0xf3,
        0xa1, 0x6a, 0xfa, 0x09, 0x73, 0xd9, 0x71, 0x97, 0x9b, 0x6b, 0xa5, 0x47, 0x61, 0xd6, 0xe2,
        0x50, 0x2c, 0x50, 0xdb, 0x76, 0xf4, 0xd2, 0x61, 0x43, 0xf0, 0x54, 0x59, 0xa4, 0x2c, 0xfd,
        0x52, 0x0d, 0x44,
    ];
    let dst: &[u8] = b"BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_AUG_";
    let point = <G1Projective as HashToCurve<ExpandMsgXmd<sha2::Sha256>>>::hash_to_curve(msg, dst);
    let expected_output: [u8; 48] = G1Affine::from(point).to_compressed();
    let result = do_basic_run(&vec![
        "run".to_string(),
        "resources/tests/bls/classic-bls-op-test-1.clsp".to_string(),
    ])
    .trim()
    .to_string();
    let hex = Bytes::new(Some(BytesFromType::Raw(expected_output.to_vec()))).hex();
    assert_eq!(result, format!("(q . 0x{hex})"));
}
