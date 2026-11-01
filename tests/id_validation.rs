use nanocrx::validate_id;

#[test]
fn accepts_valid_id() {
    assert!(validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklb"));
}

#[test]
fn rejects_id_with_invalid_letters() {
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklq"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklr"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmkls"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklt"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklu"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklv"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklw"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklx"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmkly"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklz"));
}

#[test]
fn rejects_id_with_invalid_length() {
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmkl"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklbb"));
    assert!(!validate_id(""));
}

#[test]
fn rejects_id_with_invalid_characters() {
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmkl!"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmkl1"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmkl0"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmkl$"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklФ"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklã"));
    assert!(!validate_id("iginnfkhmmfhlkagcmpgofnjhanpmkl&"));
}