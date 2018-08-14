use Config;

#[derive(Debug)]
pub struct Context {
    config: Config,
}

impl Context {
    pub fn new(config: Config) -> Context {
        Context { config }
    }
}
