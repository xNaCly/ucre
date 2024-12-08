use crate::types::uom::{
    node::{AttributeKey, AttributeValue, Attributes},
    Node, NodeKind, UcreError,
};

/// Heading signifies a Heading defined via `heading {}` in ucre lang.
///
/// This means it only holds the [Heading::text] field (not the [Node::text] method).
/// [`Heading::text`] is a fast path replacing the [Attributes::get()] call with the `text` [AttributeKey]
pub struct Heading {
    attr: Attributes,
    text: String,
}

impl Node for Heading {
    fn text(&self) -> Option<&str> {
        Some(&self.text)
    }

    fn set_text(&mut self, s: String) -> Result<(), UcreError> {
        self.text = s;
        Ok(())
    }

    fn attr(&self) -> &Attributes {
        &self.attr
    }

    fn set_attr(&mut self, k: AttributeKey, v: AttributeValue) {
        self.attr.insert(k, v);
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Heading
    }

    fn children(&self) -> Option<&Vec<Box<dyn Node>>> {
        None
    }

    fn set_children(&mut self, _: Vec<Box<dyn Node>>) -> Result<(), UcreError> {
        Err(UcreError::from_str("Heading has no children"))
    }
}
