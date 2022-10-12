#[derive(Debug, PartialEq, Eq)]
pub enum Key {
    Unknow,
}

impl Default for Key {
    fn default() -> Self {
        Self::Unknow
    }
}
