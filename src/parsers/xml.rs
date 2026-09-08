use crate::{Error, Result};
use serde::{Deserialize, Serialize};

/// Expanded element name, attributes and mixed content. Numeric text stays text.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlElement {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    pub attributes: Vec<XmlAttribute>,
    pub children: Vec<XmlNode>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlAttribute {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum XmlNode {
    Element(XmlElement),
    Text(String),
}

/// Parse well-formed XML. DTDs are disabled and output depth is limited to 128.
/// Namespace URIs and text order are retained; comments and prefix spelling are not.
pub fn parse_xml(content: &str) -> Result<XmlElement> {
    let document = roxmltree::Document::parse(content)?;
    let root = document.root_element();
    let mut stack = vec![(root.children(), element(root))];
    loop {
        match stack.last_mut().expect("open root").0.next() {
            Some(node) if node.is_element() => {
                if stack.len() >= 128 {
                    return Err(Error::InvalidInput(
                        "XML nesting exceeds 128 elements".into(),
                    ));
                }
                stack.push((node.children(), element(node)));
            }
            Some(node) if node.is_text() => {
                stack
                    .last_mut()
                    .expect("open element")
                    .1
                    .children
                    .push(XmlNode::Text(node.text().expect("text node").to_owned()));
            }
            Some(_) => {}
            None => {
                let (_, completed) = stack.pop().expect("open element");
                if let Some((_, parent)) = stack.last_mut() {
                    parent.children.push(XmlNode::Element(completed));
                } else {
                    return Ok(completed);
                }
            }
        }
    }
}

fn element(node: roxmltree::Node<'_, '_>) -> XmlElement {
    XmlElement {
        name: node.tag_name().name().to_owned(),
        namespace: node.tag_name().namespace().map(str::to_owned),
        attributes: node
            .attributes()
            .map(|attr| XmlAttribute {
                name: attr.name().to_owned(),
                namespace: attr.namespace().map(str::to_owned),
                value: attr.value().to_owned(),
            })
            .collect(),
        children: Vec::new(),
    }
}
