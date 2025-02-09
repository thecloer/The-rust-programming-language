fn main() {
    let literal = "Hello, world!";

    // literal slice = literal
    let slice: &str = &literal[7..12];
    assert_eq!(slice, "world");

    // literal full slice = source literal
    let slice: &str = &literal[..];
    assert_eq!(slice, literal);

    // literal string reference type = literal string
    let slice: &str = literal;
    assert_eq!(slice, literal);
    
    let string = String::from(literal);

    // slice of String is &str type
    let slice: &str = &string[7..12];
    assert_eq!(slice, "world");
    let slice: &str = &string[..];
    assert_eq!(slice, literal);
    let slice: &str = &string;
    assert_eq!(slice, literal);

    
    let s = String::from("Hello, world!");
    assert_eq!(&s[..], &s);
    assert_eq!(&s[7..12], "world");

    get_nth_word_test("Hello, world!");
    get_nth_word_test("Hi, how are you?");
    get_nth_word_test("Nice");
    get_nth_word_test("   The trim test   ");
    get_nth_word_test("");
    get_nth_word_test("   Duplication      spaces     test    ");
    get_nth_word_test("   \t whitespaces  \t\t \n\t  test    ");
    assert_eq!(get_nth_word("overflow test", 10), "");
}

fn get_nth_word(s: &str, n: usize) -> &str {
    // implement `s.split_whitespace().nth(n).unwrap_or("")` in only ASCII

    let trimmed_s = s.trim();
    let mut counter = 0;
    let mut start_index = 0;

    let bytes = trimmed_s.as_bytes();
    for (index, &c) in bytes.iter().enumerate() {
        if !is_whitespace(c) {
            continue;
        }

        if index > 0 && is_whitespace(bytes[index - 1]) {
            start_index = index + 1;
            continue;
        }

        if counter == n {
            return &trimmed_s[start_index..index];
        }

        counter += 1;
        start_index = index + 1;
    }
    
    if counter == n && start_index < trimmed_s.len() {
        return &trimmed_s[start_index..];
    }

    ""
}

fn is_whitespace(c: u8) -> bool {
    // c.is_whitespace()
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r'
}

fn get_nth_word_test(s: &str) {
    for (index, w) in s.split_whitespace().enumerate() {
        assert_eq!(get_nth_word(&s, index), w, "🚧 '{s}': expected [{index}] '{w}'");
    }
}