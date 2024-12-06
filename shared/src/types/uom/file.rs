use super::Node;

pub struct File {
    /// K is specifically of type String to allow custom meta_data keys
    meta_data: std::collections::HashMap<String, String>,
    /// lua holds the dynamic portion of the ucre document
    lua: String,
    doc: Vec<Box<dyn Node>>,
}
