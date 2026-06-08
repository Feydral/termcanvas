use std::collections::HashMap;

pub struct Font {
    pub glyphs: HashMap<char, Vec<u8>>,
    glyph_height: u8,
}

impl Font {
    pub fn new(glyph_height: u8) -> Self {
        Self {
            glyphs: HashMap::new(),
            glyph_height,
        }
    }

    pub fn load_from_file(path: &str, glyph_height: u8) -> Self {
        let content = std::fs::read_to_string(path).expect("Failed to read font file");

        let mut font = Self::new(glyph_height);
        font.parse(&content);
        font
    }

    fn parse(&mut self, input: &str) {
        let mut lines = input.lines().peekable();

        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if !line.starts_with('\'') {
                continue;
            }

            let ch = line.chars().nth(1).expect("missing glyph char");

            let mut bitmap = Vec::new();

            while let Some(l) = lines.next() {
                let l = l.trim();

                if l.starts_with(']') {
                    break;
                }

                for num in l.split(',') {
                    let num = num.trim();
                    if num.is_empty() {
                        continue;
                    }

                    bitmap.push(num.parse::<u8>().expect("invalid number"));
                }
            }

            self.glyphs.insert(ch, bitmap);
        }
    }

    pub fn glyph_height(&self) -> u8 {
        self.glyph_height
    }
}
