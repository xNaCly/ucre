mod err;
mod file;
mod node;
pub mod nodes;

#[allow(unused_imports)]
pub use self::err::UcreDocumentError;
pub use self::file::File;
pub use self::node::Node;

/// defines all Kinds of nodes available for usage in ucre
pub enum NodeKind {
    Heading,
    Image,
}
