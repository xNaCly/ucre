pub struct UcreDocumentError {
    cause: &'static str,
}

impl UcreDocumentError {
    pub const fn new(s: &'static str) -> Self {
        UcreDocumentError { cause: s }
    }
}
