use nanocrx::{id_from_webstore_url, slug_from_webstore_url, webstore_url_from_id_and_slug, extension_id_from_crx_id};

#[test]
fn id_from_webstore_url_test() {
    assert_eq!(id_from_webstore_url("https://chromewebstore.google.com/detail/boxel-rebound/iginnfkhmmfhlkagcmpgofnjhanpmklb?utm_source=chatgpt.com").unwrap(), "iginnfkhmmfhlkagcmpgofnjhanpmklb");
    assert_eq!(id_from_webstore_url("chromewebstore.google.com/detail/boxel-rebound/iginnfkhmmfhlkagcmpgofnjhanpmklb").unwrap(), "iginnfkhmmfhlkagcmpgofnjhanpmklb");
    assert!(id_from_webstore_url("https://chromewebstore.google.com/detail/cool-extension/invalid-id").is_err());
}

#[test]
fn slug_from_webstore_url_test() {
    assert_eq!(slug_from_webstore_url("https://chromewebstore.google.com/detail/sponsorblock-for-youtube/mnjggcdmjocbbbhaepdhchncahnbgone?utm_source=chatgpt.com").unwrap(), "sponsorblock-for-youtube");
    assert_eq!(slug_from_webstore_url("chromewebstore.google.com/detail/boxel-rebound/iginnfkhmmfhlkagcmpgofnjhanpmklb").unwrap(), "boxel-rebound");
    assert!(slug_from_webstore_url("https://chromewebstore.google.com/det").is_err());
}

#[test]
fn webstore_url_from_id_and_slug_test() {
    assert_eq!(webstore_url_from_id_and_slug("iginnfkhmmfhlkagcmpgofnjhanpmklb", "boxel-rebound").unwrap(), "https://chromewebstore.google.com/detail/boxel-rebound/iginnfkhmmfhlkagcmpgofnjhanpmklb");
    assert!(webstore_url_from_id_and_slug("invalid-id", "cool_extension").is_err());
}

#[test]
fn extension_id_from_crx_id_test() {
    assert_eq!(extension_id_from_crx_id(&[0x86, 0x8d, 0xd5, 0xa7, 0xcc, 0x57, 0xba, 0x06, 0x2c, 0xf6, 0xe5, 0xd9, 0x70, 0xdf, 0xca, 0xb1]).unwrap(), "iginnfkhmmfhlkagcmpgofnjhanpmklb");
}