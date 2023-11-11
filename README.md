# Keysymdefs

This crate provides a mapping from the keysym code to a string representation of the keysym and vice verse.


## Installation
```bash
cargo add keysymdefs
```

### Record Item
```rust
pub struct Item {
    name: &str,
    cleared_name: &str,
    keysym: u32,
    unicode: Option<u32>,
    desc: &str,
}
```

#### Get Record by KeySym
```rust
fn get_item_by_keysym(keysym: u32) -> Option<&Item>

// e.g.
if let Some(key) = get_item_by_keysym(269025044) {
    println!("{}", key.keysym());       // 269025044
    println!("{}", key.name());         // XF86XK_AudioPlay
    println!("{}", key.cleared_name()); // AudioPlay
    println!("{}", key.desc());         // Start playing of audio
    println!("{}", key.unicode());      // None
    println!("{}", key.unicode_char()); // None
}
```

#### Get Record by canonical name
```rust
fn get_item_by_name(name: &str) -> Option<&Item>
```

#### Get Record by cleared name
_truncated canonical name, no "XK\_" or "XF86XK\_" prefixes and "\_" in the middle_
```rust
fn get_item_by_cleared_name(name: &str) -> Option<&Item>
```

#### Get Record by unicode
!WARN!: Please note that not all keys have a unicode field
```rust
fn get_item_by_unicode(unicode: u32) -> Option<&Item>
```

