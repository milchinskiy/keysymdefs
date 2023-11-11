pub mod definitions;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item<'a> {
    keysym: u32,
    unicode: Option<u32>,
    name: &'a str,
    cleared_name: &'a str,
    desc: &'a str,
}

impl<'a> Item<'a> {
    /// keysym as u32
    /// e.g. 0xff55 or 65365
    pub fn keysym(&self) -> u32 {
        self.keysym
    }

    /// key name
    /// e.g. XK_Tab, XK_Return, XK_BackSpacem, XF86XK_AudioPlay, XK_Page_Up
    pub fn name(&self) -> &str {
        self.name
    }

    /// key name with truncated prefixes like XK_ or XF86XK_ and underscores
    /// e.g. Return, Tab, BackSpace, AudioPlay, PageUp
    pub fn cleared_name(&self) -> &str {
        self.cleared_name
    }

    /// unicode as u32 if exist
    pub fn unicode(&self) -> Option<u32> {
        self.unicode
    }

    /// unicode character if exist
    pub fn unicode_char(&self) -> Option<char> {
        if let Some(unicode) = self.unicode {
            return char::from_u32(unicode);
        }
        None
    }

    /// short key description if exist
    pub fn desc(&self) -> &str {
        self.desc
    }
}

/// Returns the first match or None
///
/// * `keysym`: u32 keysym (not keycode)
/// WARN: keycode is the ordinal number of the mapped key, depending on the keyboard device;
/// this is not the same as keysym! If you need key code - take a look at the xmodmap tool.
pub fn get_item_by_keysym<'a>(keysym: u32) -> Option<&'a Item<'a>> {
    definitions::KEYSYMS.iter().find(|x| x.keysym() == keysym)
}

/// Returns the first match or None
///
/// * `name`: &str key name
pub fn get_item_by_name<'a>(name: &str) -> Option<&'a Item<'a>> {
    definitions::KEYSYMS.iter().find(|x| x.name() == name)
}

/// Returns the first match or None
///
/// * `name`: name without prefix XK_ or XF86XK_
pub fn get_item_by_cleared_name<'a>(name: &str) -> Option<&'a Item<'a>> {
    definitions::KEYSYMS
        .iter()
        .find(|x| x.cleared_name() == name)
}

/// Returns the first match or None
///
/// * `unicode`: u32 of unicode
pub fn get_item_by_unicode<'a>(unicode: u32) -> Option<&'a Item<'a>> {
    definitions::KEYSYMS
        .iter()
        .find(|x| x.unicode() == Some(unicode))
}

#[test]
fn check_get_item_by_keysym() {
    assert_eq!(get_item_by_keysym(65289).unwrap().name(), "XK_Tab");
    assert_eq!(get_item_by_keysym(269025183).unwrap().name(), "XF86XK_Time");
    assert_eq!(
        get_item_by_keysym(269025148).unwrap().desc(),
        "Spell checker"
    );
    assert_eq!(get_item_by_keysym(269025140).unwrap().unicode(), None);
    assert_eq!(
        get_item_by_keysym(16780766).unwrap().clone(),
        Item {
            name: "XK_Sinh_au2",
            cleared_name: "Sinhau2",
            keysym: 16780766,
            unicode: Some(3550),
            desc: "SINHALA KOMBUVA HAA GAYANUKITTA"
        }
    );
    assert_eq!(get_item_by_keysym(1777).unwrap().unicode_char(), Some('Я'));
    assert_eq!(get_item_by_keysym(16778942).unwrap().unicode(), Some(1726));
}

#[test]
fn check_get_item_by_name() {
    assert_eq!(get_item_by_name("XK_Tab").unwrap().name(), "XK_Tab");
    assert_eq!(
        get_item_by_name("XF86XK_Time").unwrap().name(),
        "XF86XK_Time"
    );
    assert_eq!(
        get_item_by_name("XF86XK_Time").unwrap().desc(),
        "display, or shows an entry for time seeking"
    );
    assert_eq!(get_item_by_name("XF86XK_Time").unwrap().unicode(), None);
    assert_eq!(
        get_item_by_name("XK_Sinh_au2").unwrap().clone(),
        Item {
            name: "XK_Sinh_au2",
            cleared_name: "Sinhau2",
            keysym: 16780766,
            unicode: Some(3550),
            desc: "SINHALA KOMBUVA HAA GAYANUKITTA"
        }
    );
    assert_eq!(get_item_by_name("XF86XK_Time").unwrap().unicode(), None);
    assert_eq!(get_item_by_name("xK_tab"), None);
    assert_eq!(get_item_by_name("XK_Tab").unwrap().keysym(), 65289);
}

#[test]
fn check_get_item_by_cleared_name() {
    assert_eq!(
        get_item_by_cleared_name("Sinhau2").unwrap().name(),
        "XK_Sinh_au2"
    );
    assert_eq!(
        get_item_by_cleared_name("Sinhau2").unwrap().desc(),
        "SINHALA KOMBUVA HAA GAYANUKITTA"
    );
    assert_eq!(
        get_item_by_cleared_name("Sinhau2").unwrap().unicode(),
        Some(3550)
    );
    assert_eq!(
        get_item_by_cleared_name("Sinhau2").unwrap(),
        get_item_by_name("XK_Sinh_au2").unwrap()
    );
    assert_eq!(get_item_by_cleared_name("tab"), None);
    assert_eq!(get_item_by_cleared_name("Tab"), get_item_by_name("XK_Tab"));
}

#[test]
fn check_get_item_by_unicode() {
    assert_eq!(get_item_by_unicode(3550).unwrap().name(), "XK_Sinh_au2");
    assert_eq!(
        get_item_by_unicode(3550).unwrap().desc(),
        "SINHALA KOMBUVA HAA GAYANUKITTA"
    );
    assert_eq!(get_item_by_unicode(3550).unwrap().unicode(), Some(3550));
    assert_eq!(
        get_item_by_unicode(42).unwrap(),
        get_item_by_name("XK_asterisk").unwrap()
    );
    assert_eq!(get_item_by_unicode(1372).unwrap().keysym(), 16778588);
    assert_eq!(get_item_by_unicode(1726).unwrap().unicode(), Some(1726));
}
