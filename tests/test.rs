mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use std::fs::read_to_string;

    use jf::flatten;
    use serde_json::Value;


    #[test]
    fn test_mixed_json() {
        let mixed = read_to_string("tests/fixtures/mixed.json").unwrap();
        let mixed_jf = read_to_string("tests/fixtures/mixed.jf.json").unwrap();

        let json_tree: Value = serde_json::from_str(&mixed).unwrap();
        let jf_tree: Value = serde_json::from_str(&mixed_jf).unwrap();
        
        let flat_json_tree = flatten(json_tree);

        assert_eq!(flat_json_tree, jf_tree);
    }
}
