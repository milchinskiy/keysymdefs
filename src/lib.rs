pub mod definitions;
#[allow(non_upper_case_globals)]
pub mod keys;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item<'a> {
    keysym: u32,
    unicode: Option<u32>,
    name: &'a str,
    cleared_name: &'a str,
    desc: &'a str,
}

impl<'a> Item<'a> {
    pub fn new(
        keysym: u32,
        unicode: Option<u32>,
        name: &'a str,
        cleared_name: &'a str,
        desc: &'a str,
    ) -> Self {
        Self {
            keysym,
            unicode,
            name,
            cleared_name,
            desc,
        }
    }

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
