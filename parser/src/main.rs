fn main() {
    extern crate regex;
    use regex::Regex;

    let regex =
        Regex::new(r"(?m)^\#define\s([a-zA-Z_0-9]+)[\s]+(0x[0-9a-fA-F]+)(\s*\/\*\s*\(*(U\+[0-9a-fA-F]{4,6})?\s*(.*)[\s\)]\*\/)?$")
            .unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No input file specified");
    }
    let string = std::fs::read_to_string(args[1].clone()).expect("Failed to read file");
    let result = regex.captures_iter(string.as_str());

    for mat in result {
        let mut name = String::new();
        let mut cleared_name = String::new();
        let mut keysym = 0u32;
        let mut unicode: Option<u32> = None;
        let mut desc = String::new();

        if let Some(g) = mat.get(1) {
            name = g.as_str().to_string();
            cleared_name = name
                .clone()
                .trim_start_matches("XK_")
                .trim_start_matches("XF86XK_")
                .replace('_', "")
                .to_string();
        }
        if let Some(g) = mat.get(2) {
            let result = u32::from_str_radix(g.as_str().trim_start_matches("0x"), 16);
            if result.is_err() {
                continue;
            }
            keysym = result.unwrap();
        }

        if let Some(g) = mat.get(4) {
            let result = u32::from_str_radix(g.as_str().trim_start_matches("U+"), 16);
            if let Ok(result) = result {
                unicode = Some(result);
            }
        } else {
            if (0x0020..=0x007e).contains(&keysym) || (0x00a0..=0x00ff).contains(&keysym) {
                unicode = Some(keysym);
            }
            if (keysym & 0xff000000) == 0x01000000 {
                unicode = Some(keysym & 0x00ffffff);
            }
        }

        if let Some(d) = mat.get(5) {
            desc = d.as_str().trim().to_string();
        }

        println!(
            "Item {{ name: {:?}, cleared_name: {:?}, keysym: {}, unicode: {:?}, desc: {:?} }},",
            name, cleared_name, keysym, unicode, desc
        );
    }
}
