#[derive(Debug)]
pub struct Context<'ctx> {
    window: &'ctx (),
}

impl<'ctx> Context<'_> {
    fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let c = Context { window: &() };
        Ok(c)
    }
}

#[cfg(test)]
mod test {
    use super::Context;

    #[test]
    fn init() {
        let c = Context::init().expect("Failed to initialize skia context");
        c.window;
    }
}
