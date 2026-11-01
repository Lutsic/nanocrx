use std::fs;
use std::env::temp_dir;
use std::path::PathBuf;
use nanocrx::{Crx3};

fn hex(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[test]
fn standard_case() {
    let path = PathBuf::from("tests/fixtures/standard.crx");
    let crx = Crx3::open(&path).unwrap();
    let header = crx.header();

    // Header size
    assert_eq!(crx.header_size(), 1321);

    // RSA proofs
    let rsa = header.sha256_with_rsa();
    assert_eq!(rsa.len(), 2);

    // RSA proof #0
    assert_eq!(
        hex(rsa[0].public_key().unwrap()),
        "30820122300d06092a864886f70d01010105000382010f003082010a02820101008ffbbf5c3763943cb0ee01c4b5a69ab19f46746f1638a0322735ddf0716b0edcf625cbb2edeafb32d5af1e03430346f0a739db23961d65e57851f084b00e12ac0e5bdcc9d64c7c00d5b81b88333e2fdaebaaf71a75c2ae3a54de378f10d228e684794d15b4f3bd3f56d33c3f18abfc2e05c01e0831b661d0fd9f4f3f640d1793bcad41c748be0027a84d7042920554a66db8de566e204970ee103e6bd27c31bd1b6ea43c46629f086693f92a5131a8dbb59db90f73e8a0093201e97b2a8a36a0cf17b050709da2f9a46f624db6c931fcf30812ff93bd6231d81cea1a9ef581287f755ed2277ac296f59ddb18fc76dc46f057c05834c8222d2a6575a7d90862cd0203010001"
    );

    assert_eq!(
        hex(rsa[0].signature().unwrap()),
        "4f0dadba05bb59ab82d558ce0594546ad7095250a8724e4d228fe08317a23860ac2c569ed1f1d24a231b7bef146d2bf47ee38c45c50ba4b5457fe1061c014aa9a24c9edc2b8cc32be61fe46d8999b3a1bc708e62db3bbaae9393cedc1795dec204a42c8b666b5b69d0524587bd2c3b91e53deb6c51a656c4580b5bbde38e39420604fc3b9a15030d92f9fb31f1841f4c6630de6970dc75c3dadf0895b1bda47be5817e17027447cde90bbe3e8e3ae7dbb4ec6f15282c31cd99e3b32332b1f20fc10e1466180b8df994eb6d5ea0dacdef64810af632f3602ab2d78db8a8b8adbf4c715495ff51961fdccca4253bbb718d90b8f42769a23cf50687138d0286fb58"
    );

    // RSA proof #1
    assert_eq!(
        hex(rsa[1].public_key().unwrap()),
        "30820122300d06092a864886f70d01010105000382010f003082010a0282010100849cf43c61feccd0353881bc0a48613f64b6c6cd5977220ad9e53a0a19bf632b222fd400ec679130d7fe277abd2ed919d04c01dead48970b4a815ee90fc276daea65c6db7f3ab3a5a1cb5639b835d8a89663d0b6dd36da51b5902c3439e3754945bfcf015f1775daba433e7b975a5aee9462e335e53198688c1e62909a4ca726a2ee112f229c3c8a9c3ef3fce8992d64f0324fc4719a60e530bd47d0abf7d9c4bf9e3fb67e13f0b0be9cb671a4fd1332cd49c4a12046d9d24c72f64f72af5c082b373ed6f1731a206a0780366973ebac23b212866572952d66763fa3983956aceaf5beadcc7a74c0c99575f5591bdfe9c7e1c6d3331b2d331b8cc3ec981855390203010001"
    );

    assert_eq!(
        hex(rsa[1].signature().unwrap()),
        "380d8ef2139a189bfcf74ff6cb784cbc31a2c8af989b4b8d8f39dd001be5829c215323c68128e2f1470a4f9694003592bab610a1f969e1f06760732857995fa64749e0987ad4bed2539fa916783c4ade953e9f23acab3908a87a9d44c2b84e4b35e80f6a03823aa870fb4576885f3a908adac04c60ec0dd3d779d1f0213f18196bdd0bc5b6f7d394bc44775525082c2fcad60b750cd50f2c75d7cce4f5014431d330a4ddddc49380815d988eb53ac7ca51fdb04bd507e29eaed34e6cae7d1dd777594ff99d25a192f4eacd7cba5845cd5e9d1da460f7e24f35222833511a8e64b23a9f936a98b443defbdd4467356d6584072e9775aabf067b35141f8224ccbf"
    );

    // ECDSA proofs
    let ecdsa = header.sha256_with_ecdsa();
    assert_eq!(ecdsa.len(), 1);

    // ECDSA proof #0
    assert_eq!(
        hex(ecdsa[0].public_key().unwrap()),
        "3059301306072a8648ce3d020106082a8648ce3d03010703420004b402f9a91caee10c84f8dc9b177298a5e032d5db2b59d849c0efbee86bf9f86252b86a35536c078b38d4ff9addbcddf9bc482269cc2d6ca10160b451e37bd1cf"
    );

    assert_eq!(
        hex(ecdsa[0].signature().unwrap()),
        "3045022068aae5df13446dc7b41ecbedf6ef3e90e4ea36155752cd7e5f7247a4e36854280221008709aab74ab1a0304e40f36299c7dae9c5629d6c65ebc6834880debe728bdf4d"
    );

    // Verified contents
    assert!(header.verified_contents().is_none());

    // Signed header data
    assert_eq!(
        hex(header.signed_header_data().unwrap()),
        "0a10868dd5a7cc57ba062cf6e5d970dfcab1"
    );

    // CRX ID
    assert_eq!(
        hex(header.crx_id().unwrap()),
        "868dd5a7cc57ba062cf6e5d970dfcab1"
    );

    assert_eq!(
        header.extension_id().unwrap(),
        "iginnfkhmmfhlkagcmpgofnjhanpmklb"
    );
}