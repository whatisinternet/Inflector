use numbers::ordinalize::*;

#[test]
fn should_return_the_orgional_string_if_not_a_number_ordinalize(){
    let mock_string: String = "a".to_string();
    let expected_string: String = "a".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_0_as_0th(){
    let mock_string: String = "0".to_string();
    let expected_string: String = "0th".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_1_as_1st(){
    let mock_string: String = "1".to_string();
    let expected_string: String = "1st".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_2_as_2nd(){
    let mock_string: String = "2".to_string();
    let expected_string: String = "2nd".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_3_as_3rd(){
    let mock_string: String = "3".to_string();
    let expected_string: String = "3rd".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_9_as_9th(){
    let mock_string: String = "9".to_string();
    let expected_string: String = "9th".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_12_as_12th(){
    let mock_string: String = "12".to_string();
    let expected_string: String = "12th".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_12000_as_12000th(){
    let mock_string: String = "12000".to_string();
    let expected_string: String = "12000th".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_12001_as_12001th(){
    let mock_string: String = "12001".to_string();
    let expected_string: String = "12001st".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_12002_as_12002nd(){
    let mock_string: String = "12002".to_string();
    let expected_string: String = "12002nd".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_12003_as_12003th(){
    let mock_string: String = "12003".to_string();
    let expected_string: String = "12003rd".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test]
fn should_ordinalize_12004_as_12004th(){
    let mock_string: String = "12004".to_string();
    let expected_string: String = "12004th".to_string();
    let asserted_string: String = ordinalize(mock_string);
    assert!(asserted_string == expected_string);
}
