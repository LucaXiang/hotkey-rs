#[derive(Debug, PartialEq, Eq)]
pub enum Modifier {
    Unknow,
}

impl Default for Modifier {
    fn default() -> Self {
        Self::Unknow
    }
}
