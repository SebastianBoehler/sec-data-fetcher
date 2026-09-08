use sec_data_fetcher::{XmlNode, extract_tables, parse_xml};

#[test]
fn extracts_tables_with_html5_recovery_and_nested_ownership() {
    assert_eq!(
        extract_tables(include_str!("fixtures/tables.html")),
        vec![
            vec![
                vec!["Metric", "Value"],
                vec!["Cash & equivalents", "1,234"],
                vec!["ParentChild", "Value"]
            ],
            vec![vec!["Child"]],
            vec![vec!["Unclosed", "Cells"]],
        ]
    );
    assert!(extract_tables("<p>No tables</p>").is_empty());
}

#[test]
fn preserves_xml_namespaces_identifiers_and_mixed_text_order() {
    let xml = parse_xml(include_str!("fixtures/document.xml")).unwrap();
    assert_eq!(xml.name, "filing");
    assert_eq!(xml.namespace.as_deref(), Some("urn:filing"));
    assert_eq!(
        xml.attributes[0].namespace.as_deref(),
        Some("urn:identifier")
    );
    assert_eq!(xml.attributes[0].value, "0000123");
    assert_eq!(xml.children[0], XmlNode::Text("before".into()));
    let XmlNode::Element(value) = &xml.children[1] else {
        panic!("expected value element")
    };
    assert_eq!(value.children[0], XmlNode::Text("000042".into()));
    assert_eq!(xml.children[2], XmlNode::Text("after & more".into()));
    assert_eq!(xml.children.len(), 4);
}

#[test]
fn rejects_malformed_xml_dtd_entities_and_excessive_depth() {
    for xml in [
        "<a><b></a>",
        "<a/><b/>",
        "<!DOCTYPE a [<!ENTITY e 'expanded'>]><a>&e;</a>",
        "<a>&unknown;</a>",
        "<!DOCTYPE a><a/>",
    ] {
        assert!(parse_xml(xml).is_err(), "accepted {xml}");
    }
    for depth in [129, 10_000] {
        let xml = format!("{}{}", "<a>".repeat(depth), "</a>".repeat(depth));
        assert!(parse_xml(&xml).is_err());
    }
    let boundary = format!("{}{}", "<a>".repeat(128), "</a>".repeat(128));
    let tree = parse_xml(&boundary).unwrap();
    assert!(serde_json::to_string(&tree).is_ok());
}
