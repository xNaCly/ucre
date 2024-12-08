use std::collections::HashMap;

use super::NodeKind;
use super::UcreError;

pub type AttributeKey = String;
pub type AttributeValue = Vec<String>;
pub type Attributes = HashMap<AttributeKey, AttributeValue>;

/// Node holds a singular node, its attributes, its children and other meta_data inside the UOM (ucre object model)
///
/// A Node is written in ucre lang as follows:
///
///     node_name {
///         attribute_name attribute_value attribute_value
///         ...
///     }
///
/// Said node and its attributes can be modified via Node::set_*, while the access is done by Node::*
pub trait Node {
    /// extract the inner text
    fn text(&self) -> Option<&str>;
    /// sets the contained type to s, if not supported, UcreDocumentError with cause dispatched
    fn set_text(&mut self, s: String) -> Result<(), UcreError>;

    /// returns all attributes, K is specifically of type String to allow custom attribute keys
    fn attr(&self) -> &Attributes;
    /// sets attribute k with v, if k already set the value is overwritten to v
    fn set_attr(&mut self, k: AttributeKey, v: AttributeValue);

    /// returns all children
    fn children(&self) -> Option<&Vec<Box<dyn Node>>>;
    /// attempts to set the children of the node to c, if not supported, UcreDocumentError with cause dispatched
    fn set_children(&mut self, c: Vec<Box<dyn Node>>) -> Result<(), UcreError>;

    fn kind(&self) -> NodeKind;
}
