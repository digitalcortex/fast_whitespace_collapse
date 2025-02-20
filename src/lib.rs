use wide::u8x16;


/// Collapses consecutive spaces and tabs into a single space in the input string.
///
/// This function efficiently processes input using SIMD (`u8x16`) for performance.
/// It removes redundant whitespace while preserving single spaces between words.
/// Tabs (`\t`) are treated as spaces.
///
/// # Parameters
/// - `input`: A string slice (`&str`) containing text with irregular spacing.
///
/// # Returns
/// - A `String` with collapsed whitespace, where multiple spaces/tabs are replaced by a single space.
///
/// # Example
/// ```
/// use fast_whitespace_collapse::collapse_whitespace;
/// let input = "This   is \t  a   test.";
/// let output = collapse_whitespace(input);
/// assert_eq!(output, "This is a test.");
/// ```
///
/// # Performance
/// - Uses SIMD (`u8x16`) to process 16 bytes at a time.
/// - Falls back to scalar processing for remaining bytes.
/// - Ensures valid UTF-8 output by keeping only original characters.
pub fn collapse_whitespace(input: &str) -> String {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut result = Vec::with_capacity(len);

    let space = u8x16::splat(b' ');
    let tab   = u8x16::splat(b'\t');

    let mut i = 0;
    let mut last_was_space = true;

    while i + 16 <= len {
        // Load a 16-byte chunk
        let arr: [u8; 16] = bytes[i..i+16].try_into().unwrap();
        let chunk = u8x16::from(arr);

        // Compare each lane to space or tab
        let cmp_space = chunk.cmp_eq(space);
        let cmp_tab   = chunk.cmp_eq(tab);
        let cmp_any   = cmp_space | cmp_tab;

        // Convert to arrays
        let chunk_arr = chunk.to_array();
        let mask_arr  = cmp_any.to_array();

        // mask_arr lane is 0xFF if that lane is space or tab, else 0x00
        for (&byte, &mask_byte) in chunk_arr.iter().zip(mask_arr.iter()) {
            let is_whitespace = mask_byte == 0xFF;
            if is_whitespace {
                if !last_was_space {
                    result.push(b' ');
                    last_was_space = true;
                }
            } else {
                result.push(byte);
                last_was_space = false;
            }
        }

        i += 16;
    }

    // Handle leftover bytes (scalar pass)
    while i < len {
        let b = bytes[i];
        if b == b' ' || b == b'\t' {
            if !last_was_space {
                result.push(b' ');
                last_was_space = true;
            }
        } else {
            result.push(b);
            last_was_space = false;
        }
        i += 1;
    }

    // Remove trailing space if any
    if result.last() == Some(&b' ') {
        result.pop();
    }

    // Safety: We only push valid UTF-8 bytes
    unsafe { String::from_utf8_unchecked(result) }
}

#[cfg(test)]
mod tests {
    use super::collapse_whitespace;
    
    #[test]
    fn test_basic_collapse() {
        assert_eq!(collapse_whitespace("This   is 	  a   test."), "This is a test.");
        assert_eq!(collapse_whitespace("  Leading and trailing  "), "Leading and trailing");
        assert_eq!(collapse_whitespace("Multiple     spaces"), "Multiple spaces");
        assert_eq!(collapse_whitespace("NoExtraSpaces"), "NoExtraSpaces");
        assert_eq!(collapse_whitespace("   "), "");
    }
    
    #[test]
    fn test_unicode_characters() {
        assert_eq!(collapse_whitespace("こんにちは  世界"), "こんにちは 世界"); // Japanese
        assert_eq!(collapse_whitespace("Привет    мир"), "Привет мир"); // Cyrillic
        assert_eq!(collapse_whitespace("你好  世界"), "你好 世界"); // Chinese
        assert_eq!(collapse_whitespace("안녕하세요    세계"), "안녕하세요 세계"); // Korean
        assert_eq!(collapse_whitespace("😀  😃  😄"), "😀 😃 😄"); // Emojis
    }
    
    #[test]
    fn test_mixed_whitespace() {
        assert_eq!(collapse_whitespace("Mix  of	tabs and spaces"), "Mix of tabs and spaces");
        assert_eq!(collapse_whitespace("  	   Multiple types of   whitespace  	  "), "Multiple types of whitespace");
    }
    
    #[test]
    fn test_newlines_not_collapsed() {
        assert_eq!(collapse_whitespace("Line1\n   Line2\nLine3"), "Line1\n Line2\nLine3");
        assert_eq!(collapse_whitespace("First line\n    Second line"), "First line\n Second line");
    }

    #[test]
    fn single_tab() {
        assert_eq!(collapse_whitespace("\t"), "");
        assert_eq!(collapse_whitespace("A\tB"), "A B");
    }

    #[test]
    fn only_tabs() {
        assert_eq!(collapse_whitespace("\t\t\t"), "");
        assert_eq!(collapse_whitespace("\t\t\tA\t\tB\t\t"), "A B");
    }

    #[test]
    fn empty_input() {
        assert_eq!(collapse_whitespace(""), "");
    }

    #[test]
    fn mixed_whitespace_around_text() {
        assert_eq!(collapse_whitespace("\t  Surround \t"), "Surround");
    }

    #[test]
    fn unicode_with_tabs() {
        assert_eq!(collapse_whitespace("こんにちは\t\t世界"), "こんにちは 世界");
        assert_eq!(collapse_whitespace("你好\t世界\t"), "你好 世界");
    }
}