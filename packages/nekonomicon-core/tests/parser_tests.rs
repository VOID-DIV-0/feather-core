use nekonomicon_core::parser::{parser, AbstractTreeNode, DataNode};

#[test]
fn parses_simple_module() {
    let input = "say 'hello'.";
    let nodes = parser(input);
    assert!(nodes.iter().any(|n| matches!(n, AbstractTreeNode::Module(m) if m.name == "say")));
}

#[test]
fn parses_say_literal() {
    let input = "say @my_variable.";
    let nodes = parser(input);
    assert!(nodes.iter().any(|n| matches!(n, AbstractTreeNode::Module(m) if m.name == "say")));
}

#[test]
fn parses_data_text() {
    let input = "text 'abc'.";
    let nodes = parser(input);
    let found = nodes.iter().any(|n| match n {
        AbstractTreeNode::Module(m) => m.args.iter().any(|a| matches!(a, AbstractTreeNode::Data(DataNode::Text(t)) if t == "abc")),
        _ => false,
    });
    assert!(found);
}




mod signature_tests {
    use nekonomicon_core::parser::{parser, AbstractTreeNode, SignatureNode};

    #[test]
    fn parses_signatures() {
        let input = "say 'hi' trace elapsed silent timeout 5 sec on mac.";
        let nodes = parser(input);
        let found_trace = nodes.iter().any(|n| matches!(n, AbstractTreeNode::Signature(SignatureNode::Trace)));
        let found_elapsed = nodes.iter().any(|n| matches!(n, AbstractTreeNode::Signature(SignatureNode::Elapsed)));
        let found_silent = nodes.iter().any(|n| matches!(n, AbstractTreeNode::Signature(SignatureNode::Silent)));
        let found_timeout = nodes.iter().any(|n| matches!(n, AbstractTreeNode::Signature(SignatureNode::Timeout(v, u)) if *v == 5.0 && u == "sec"));
        let found_on = nodes.iter().any(|n| matches!(n, AbstractTreeNode::Signature(SignatureNode::On(p)) if p == "mac"));
        assert!(found_trace && found_elapsed && found_silent && found_timeout && found_on);
    }

    fn assert_signature(input: &str, expected: &[SignatureNode]) {
        let nodes = parser(input);
        for sig in expected {
            assert!(
                nodes.iter().any(|n| matches!(n, AbstractTreeNode::Signature(s) if s == sig)),
                "Signature {:?} not found in input '{}'", sig, input
            );
        }
    }

    #[test]
    fn parses_various_signatures() {
        let cases = vec![
            ("trace.", vec![SignatureNode::Trace]),
            ("elapsed.", vec![SignatureNode::Elapsed]),
            ("silent.", vec![SignatureNode::Silent]),
            ("timeout 10 sec.", vec![SignatureNode::Timeout(10.0, "sec".to_string())]),
            ("on linux.", vec![SignatureNode::On("linux".to_string())]),
            ("trace elapsed.", vec![SignatureNode::Trace, SignatureNode::Elapsed]),
            ("trace silent elapsed.", vec![SignatureNode::Trace, SignatureNode::Silent, SignatureNode::Elapsed]),
            ("timeout 3 min on windows.", vec![SignatureNode::Timeout(3.0, "min".to_string()), SignatureNode::On("windows".to_string())]),
            // Add more combinations as needed
        ];

        for (input, expected) in cases {
            assert_signature(input, &expected);
        }
    }

    
}