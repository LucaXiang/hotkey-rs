mod modifier;
pub use modifier::Modifier;
mod key;
pub use key::Key;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Hotkey {
    modifiers: Vec<Modifier>,
    key: Key,
}

impl Hotkey {
    pub fn new(modifiers: Vec<Modifier>, key: Key) -> Self {
        Hotkey { modifiers, key }
    }
}

#[cfg(test)]
mod tests {
    use crate::Hotkey;
    use crate::Key;
    #[test]
    fn it_works() {
        {
            let actual = Hotkey::default();
            let expect = Hotkey::new(vec![], Key::default());
            assert_eq!(actual, expect);
        }
    }
}
