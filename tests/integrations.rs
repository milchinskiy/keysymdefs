use keysymdefs::{
    get_item_by_cleared_name, get_item_by_keysym, get_item_by_name, get_item_by_unicode, keys, Item,
};

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
        get_item_by_keysym(keys::XK_Sinh_au2).unwrap().clone(),
        Item::new(
            16780766,
            Some(3550),
            "XK_Sinh_au2",
            "Sinhau2",
            "SINHALA KOMBUVA HAA GAYANUKITTA"
        )
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
        Item::new(
            16780766,
            Some(3550),
            "XK_Sinh_au2",
            "Sinhau2",
            "SINHALA KOMBUVA HAA GAYANUKITTA"
        )
    );
    assert_eq!(get_item_by_name("XF86XK_Time").unwrap().unicode(), None);
    assert_eq!(get_item_by_name("xK_tab"), None);
    assert_eq!(get_item_by_name("XK_Tab").unwrap().keysym(), keys::XK_Tab);
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

#[test]
fn check_keysym() {
    assert_eq!(
        get_item_by_keysym(keys::XK_Hyper_L).unwrap().keysym(),
        keys::XK_Hyper_L
    );
    assert_ne!(
        get_item_by_keysym(keys::XF86XK_AudioPlay).unwrap().keysym(),
        keys::XF86XK_AudioStop
    );
    assert_eq!(
        get_item_by_keysym(keys::XF86XK_AudioPlay).unwrap().keysym(),
        keys::XF86XK_AudioPlay
    );
    assert_eq!(keys::XF86XK_AudioPlay, 269025044);
    assert_eq!(keys::XF86XK_AudioPlay, 0x1008ff14);
}
