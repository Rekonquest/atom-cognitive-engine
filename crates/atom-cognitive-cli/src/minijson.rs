//! A minimal, strict JSON parser (house law: zero third-party dependencies).
//!
//! Supports exactly what the Mother corpus contains: objects, arrays,
//! strings (with escapes), integers, booleans, and null. Rejects anything
//! else fail-closed — including trailing content after the top-level value.

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(text) => Some(text),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Json::Int(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Json::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Json]> {
        match self {
            Json::Array(items) => Some(items),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    position: usize,
}

/// Parse one JSON document; trailing non-whitespace content is an error.
pub fn parse(text: &str) -> Result<Json, String> {
    let mut parser = Parser {
        bytes: text.as_bytes(),
        position: 0,
    };
    let value = parser.value()?;
    parser.skip_whitespace();
    if parser.position != parser.bytes.len() {
        return Err(format!("trailing content at byte {}", parser.position));
    }
    Ok(value)
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\t' | b'\r')) {
            self.position += 1;
        }
    }

    fn expect(&mut self, byte: u8) -> Result<(), String> {
        if self.peek() == Some(byte) {
            self.position += 1;
            Ok(())
        } else {
            Err(format!(
                "expected {:?} at byte {}, found {:?}",
                byte as char,
                self.position,
                self.peek().map(|b| b as char)
            ))
        }
    }

    fn literal(&mut self, word: &str) -> Result<(), String> {
        let bytes = word.as_bytes();
        if self.bytes.len() >= self.position + bytes.len()
            && &self.bytes[self.position..self.position + bytes.len()] == bytes
        {
            self.position += bytes.len();
            Ok(())
        } else {
            Err(format!(
                "expected literal {word:?} at byte {}",
                self.position
            ))
        }
    }

    fn value(&mut self) -> Result<Json, String> {
        self.skip_whitespace();
        match self.peek() {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') => {
                self.literal("true")?;
                Ok(Json::Bool(true))
            }
            Some(b'f') => {
                self.literal("false")?;
                Ok(Json::Bool(false))
            }
            Some(b'n') => {
                self.literal("null")?;
                Ok(Json::Null)
            }
            Some(byte) if byte == b'-' || byte.is_ascii_digit() => self.number(),
            other => Err(format!(
                "unexpected {:?} at byte {}",
                other.map(|b| b as char),
                self.position
            )),
        }
    }

    fn object(&mut self) -> Result<Json, String> {
        self.expect(b'{')?;
        let mut entries = Vec::new();
        self.skip_whitespace();
        if self.peek() == Some(b'}') {
            self.position += 1;
            return Ok(Json::Object(entries));
        }
        loop {
            self.skip_whitespace();
            let key = self.string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            let value = self.value()?;
            entries.push((key, value));
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => {
                    self.position += 1;
                }
                Some(b'}') => {
                    self.position += 1;
                    return Ok(Json::Object(entries));
                }
                other => {
                    return Err(format!(
                        "expected ',' or '}}' at byte {}, found {:?}",
                        self.position,
                        other.map(|b| b as char)
                    ));
                }
            }
        }
    }

    fn array(&mut self) -> Result<Json, String> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_whitespace();
        if self.peek() == Some(b']') {
            self.position += 1;
            return Ok(Json::Array(items));
        }
        loop {
            let value = self.value()?;
            items.push(value);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => {
                    self.position += 1;
                }
                Some(b']') => {
                    self.position += 1;
                    return Ok(Json::Array(items));
                }
                other => {
                    return Err(format!(
                        "expected ',' or ']' at byte {}, found {:?}",
                        self.position,
                        other.map(|b| b as char)
                    ));
                }
            }
        }
    }

    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            let byte = self
                .peek()
                .ok_or_else(|| "unterminated string".to_string())?;
            match byte {
                b'"' => {
                    self.position += 1;
                    return Ok(out);
                }
                b'\\' => {
                    self.position += 1;
                    let escape = self
                        .peek()
                        .ok_or_else(|| "unterminated escape".to_string())?;
                    self.position += 1;
                    match escape {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let hex = self
                                .bytes
                                .get(self.position..self.position + 4)
                                .ok_or_else(|| "short unicode escape".to_string())?;
                            let text = std::str::from_utf8(hex)
                                .map_err(|_| "invalid unicode escape".to_string())?;
                            let code = u32::from_str_radix(text, 16)
                                .map_err(|_| "invalid unicode escape".to_string())?;
                            self.position += 4;
                            out.push(char::from_u32(code).unwrap_or('\u{fffd}'));
                        }
                        other => {
                            return Err(format!(
                                "invalid escape {:?} at byte {}",
                                other as char, self.position
                            ));
                        }
                    }
                }
                _ => {
                    // Consume one UTF-8 code point.
                    let start = self.position;
                    let len = utf8_len(byte);
                    let end = (start + len).min(self.bytes.len());
                    let text = std::str::from_utf8(&self.bytes[start..end])
                        .map_err(|_| "invalid UTF-8 in string".to_string())?;
                    out.push_str(text);
                    self.position = end;
                }
            }
        }
    }

    fn number(&mut self) -> Result<Json, String> {
        let start = self.position;
        if self.peek() == Some(b'-') {
            self.position += 1;
        }
        while matches!(self.peek(), Some(byte) if byte.is_ascii_digit()) {
            self.position += 1;
        }
        // Fractions/exponents are rejected: the corpus contains integers
        // only, and silent float truncation would be a lie.
        if matches!(self.peek(), Some(b'.' | b'e' | b'E')) {
            return Err(format!(
                "non-integer number at byte {start} (integers only by design)"
            ));
        }
        let text = std::str::from_utf8(&self.bytes[start..self.position])
            .map_err(|_| "invalid number".to_string())?;
        text.parse::<i64>()
            .map(Json::Int)
            .map_err(|_| format!("invalid integer {text:?}"))
    }
}

fn utf8_len(byte: u8) -> usize {
    match byte {
        0x00..=0x7f => 1,
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        _ => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_corpus_shape() {
        let text = r#"{"id": "ANCHOR-001", "bucket": 3, "load_bearing": true, "behaviors_encoded": ["P-005", "C-001"], "echoes_to": ["B9-001"], "cast_present": ["grandmother"], "note": null}"#;
        let value = parse(text).expect("parses");
        assert_eq!(value.get("id").and_then(Json::as_str), Some("ANCHOR-001"));
        assert_eq!(value.get("bucket").and_then(Json::as_int), Some(3));
        assert_eq!(
            value.get("load_bearing").and_then(Json::as_bool),
            Some(true)
        );
        let behaviors = value
            .get("behaviors_encoded")
            .and_then(Json::as_array)
            .expect("array");
        assert_eq!(behaviors.len(), 2);
        assert_eq!(behaviors[0].as_str(), Some("P-005"));
        assert_eq!(value.get("note"), Some(&Json::Null));
    }

    #[test]
    fn handles_escapes_and_unicode() {
        let value = parse(r#""flour \u2014 warm\nair""#).expect("parses");
        assert_eq!(value.as_str(), Some("flour — warm\nair"));
    }

    #[test]
    fn rejects_trailing_content_and_floats() {
        assert!(parse("{} extra").is_err());
        assert!(parse("[1, 2.5]").is_err());
        assert!(parse("[1, 2,]").is_err());
        assert!(parse("{\"a\": }").is_err());
        assert!(parse("").is_err());
    }
}
