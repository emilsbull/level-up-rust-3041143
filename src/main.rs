mod run_length_encoding {
    pub fn encode(text: &str, length_first: bool) -> String {
        let mut count = 0;
        let mut previous = None;
        let mut encoded = String::new();
        let chars = text.chars();

        for c in chars {
            if Some(c) == previous && count < 9 {
                count += 1;
            } else {
                if let Some(p) = previous {
                    if length_first {
                        encoded.push_str(&format!("{}{}", count, p));
                    } else {
                        encoded.push_str(&format!("{}{}", p, count));
                    }
                }
                previous = Some(c);
                count = 1;
            }
        }
        if let Some(p) = previous {
            if length_first {
                encoded.push_str(&format!("{}{}", count, p));
            } else {
                encoded.push_str(&format!("{}{}", p, count));
            }
        }
        encoded
    }

    pub fn decode(text: &str, length_first: bool) -> String {
        let mut decoded = String::new();
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if length_first {
                let mut count_str = String::new();
                while c.is_ascii_digit() {
                    count_str.push(c);
                    if let Some(next) = chars.peek() {
                        if next.is_ascii_digit() {
                            chars.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                let count: usize = count_str.parse().unwrap_or(1);
                if let Some(next_char) = chars.next() {
                    decoded.push_str(&next_char.to_string().repeat(count));
                }
            } else {
                let char_to_repeat = c;
                let mut count_str = String::new();
                while let Some(next) = chars.peek() {
                    if next.is_ascii_digit() {
                        count_str.push(*next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let count: usize = count_str.parse().unwrap_or(1);
                decoded.push_str(&char_to_repeat.to_string().repeat(count));
            }
        }
        decoded
    }
}

fn main() {
    use run_length_encoding::*;
    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    println!("{}", encode(input, true));
    println!("{}", decode(&encode(input, true), true));
    println!("{}", encode(input, false));
    println!("{}", decode(&encode(input, false), false));
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc", true), "1a1b1c");
    assert_eq!(encode("abc", false), "a1b1c1");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input, true));
    assert_eq!(decode(&encode(input, true), true), input);

    println!("{}", encode(input, false));
    assert_eq!(decode(&encode(input, false), false), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input, true), "5A1 9A1A1 9A9A2A");
    assert_eq!(encode(input, false), "A5 1A9A1 1A9A9A2");
}
