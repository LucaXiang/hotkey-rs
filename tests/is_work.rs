use hotkey_rs::{Hotkey, Key, Modifier};

#[test]
fn it_works() {
    {
        let actual = Hotkey::default();
        let expect = Hotkey::new(vec![], Key::default());
        assert_eq!(actual, expect);
    }
    {
        let actual = Hotkey::new(vec![Modifier::Unknow], Key::Unknow);
        let expect = Hotkey::new(vec![Modifier::Unknow], Key::default());
        assert_eq!(actual, expect);
    }
}
