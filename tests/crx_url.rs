use nanocrx::CrxUrl;

#[test]
fn standard_case_test() {
    let url = CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb")
        .expect("REASON")
        .prodversion("155.0.0.0")
        .expect("REASON")
        .allow_crx2(true)
        .to_url();

    assert_eq!(url, "https://clients2.google.com/service/update2/crx?response=redirect&prodversion=155.0.0.0&acceptformat=crx2,crx3&x=id%3Diginnfkhmmfhlkagcmpgofnjhanpmklb%26installsource%3Dondemand%26uc");
}