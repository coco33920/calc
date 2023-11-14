use std::str::FromStr;

use crate::lexing::token::Operator::*;
use crate::lexing::token::Token;

pub fn is_an_allowed_char(character: char) -> bool {
    character.is_alphanumeric()
        || character == '+'
        || character == '-'
        || character == '*'
        || character == '/'
        || character == '('
        || character == ')'
        || character == '"'
        || character == '.'
        || character == '='
        || character == '^'
}

fn lex_int(
    current_char: char,
    chars: &mut Vec<char>,
    current_pos: usize,
    len: usize,
) -> (i64, usize) {
    let (a, b) = lex_raddix(current_char, chars, current_pos, len);
    let err = i64::from_str(&a);
    if err.is_err() {
        (0, b)
    } else {
        (err.unwrap(), b)
    }
}

fn lex_raddix(
    mut current_char: char,
    chars: &mut Vec<char>,
    mut current_pos: usize,
    len: usize,
) -> (String, usize) {
    let mut str: String = String::new();
    while current_pos < len && (current_char.is_ascii_digit()) {
        str += &*current_char.to_string();

        current_pos += 1;
        let a = chars.get(current_pos);
        match a {
            Some(t) => current_char = *t,
            None => break,
        }
    }
    (str, current_pos)
}

fn lex_string(
    mut current_char: char,
    chars: &mut Vec<char>,
    mut current_pos: usize,
    len: usize,
) -> (String, usize) {
    let mut str: String = String::new();
    while current_pos < len && current_char.is_alphanumeric() {
        str += &*current_char.to_string();

        current_pos += 1;
        let a = chars.get(current_pos);
        match a {
            Some(t) => current_char = *t,
            None => break,
        }
    }
    (str, current_pos)
}

fn lex_float(
    whole_side: i64,
    chars: &mut Vec<char>,
    mut current_pos: usize,
    len: usize,
) -> (f64, usize) {
    current_pos += 1;
    let current_char_options = chars.get(current_pos);
    let current_char = match current_char_options {
        Some(t) => t,
        None => &'0',
    };
    let (a, b) = lex_raddix(*current_char, chars, current_pos, len);
    let f = f64::from_str(&*(whole_side.to_string().as_str().to_owned() + "." + a.as_str()));
    if f.is_err() {
        return (f64::NAN, b);
    }
    (f.unwrap(), b)
}

pub fn lex(input: String) -> Vec<Token> {
    let mut vec: Vec<Token> = Vec::new();

    let mut current_pos = 0;

    let mut chars = input.as_str().chars().collect::<Vec<char>>();

    let length = input.len();
    while current_pos < input.len() {
        let current_character: char = chars.get(current_pos).unwrap().to_ascii_lowercase();
        if !is_an_allowed_char(current_character) {
            current_pos += 1;
            continue;
        };

        match current_character {
            '+' => {
                vec.push(Token::OPE(PLUS));
                current_pos += 1
            }
            '-' => {
                vec.push(Token::OPE(MINUS));
                current_pos += 1
            }
            '*' => {
                vec.push(Token::OPE(MULTIPLICATION));
                current_pos += 1
            }
            '/' => {
                vec.push(Token::OPE(DIVIDE));
                current_pos += 1
            }
            ')' => {
                vec.push(Token::RPAR);
                current_pos += 1
            }
            '(' => {
                vec.push(Token::LPAR);
                current_pos += 1
            }
            '=' => {
                vec.push(Token::EQUAL);
                current_pos += 1
            }
            '^' => {
                vec.push(Token::OPE(EXPO));
                current_pos += 1
            }
            ch => {
                if ch.is_numeric() {
                    let (a, b) = lex_int(current_character, &mut chars, current_pos, length);
                    current_pos = b;
                    let cha = chars.get(current_pos);
                    match cha {
                        Some(char) => {
                            if *char == '.' {
                                let (a1, b1) = lex_float(a, &mut chars, current_pos, length);
                                current_pos = b1;
                                vec.push(Token::FLOAT(a1))
                            } else {
                                vec.push(Token::INT(a));
                                current_pos = b;
                            }
                        }
                        None => {
                            vec.push(Token::INT(a));
                            current_pos = b;
                        }
                    }
                }
                if ch.is_alphabetic() {
                    let (a, b) = lex_string(current_character, &mut chars, current_pos, length);
                    current_pos = b;
                    vec.push(Token::IDENTIFIER(a))
                }
                if ch == '.' {
                    let (a, b) = lex_float(0, &mut chars, current_pos, length);
                    current_pos = b;
                    vec.push(Token::FLOAT(a))
                }
            }
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use crate::lexing::lexer::lex;
    use crate::lexing::token::Operator::*;
    use crate::lexing::token::Token::*;

    #[test]
    fn lex_plus() {
        let mut expected = Vec::new();
        expected.push(OPE(PLUS));
        let result = lex("+".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_minus() {
        let mut expected = Vec::new();
        expected.push(OPE(MINUS));
        let result = lex("-".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_mult() {
        let mut expected = Vec::new();
        expected.push(OPE(MULTIPLICATION));
        let result = lex("*".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_divide() {
        let mut expected = Vec::new();
        expected.push(OPE(DIVIDE));
        let result = lex("/".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_operators() {
        let mut expected = Vec::new();
        expected.push(OPE(PLUS));
        expected.push(OPE(MULTIPLICATION));
        expected.push(OPE(MINUS));
        expected.push(OPE(DIVIDE));
        let result = lex("+*-/".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_lpar() {
        let mut expected = Vec::new();
        expected.push(LPAR);
        let result = lex("(".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_rpar() {
        let mut expected = Vec::new();
        expected.push(RPAR);
        let result = lex(")".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_equal() {
        let mut expected = Vec::new();
        expected.push(EQUAL);
        let result = lex("=".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_tokens() {
        let mut expected = Vec::new();
        expected.push(LPAR);
        expected.push(RPAR);
        expected.push(EQUAL);
        let result = lex("()=".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_simple_int() {
        let mut expected = Vec::new();
        expected.push(INT(1));
        let result = lex("1".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_complex_int() {
        let mut expected = Vec::new();
        expected.push(INT(100));
        let result = lex("100".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_simple_string() {
        let mut expected = Vec::new();
        expected.push(IDENTIFIER("test".to_string()));
        let result = lex("test".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_operation() {
        let mut expected = Vec::new();
        expected.push(INT(1));
        expected.push(OPE(PLUS));
        expected.push(INT(1));
        let result = lex("1 + 1".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_equality() {
        let mut expected = Vec::new();
        expected.push(IDENTIFIER("var1".to_string()));
        expected.push(EQUAL);
        expected.push(INT(100));
        let result = lex("var1 = 100".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn test_simple_float() {
        let mut expected = Vec::new();
        expected.push(FLOAT(0.14));
        let result = lex(".14".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_float() {
        let mut expected = Vec::new();
        expected.push(FLOAT(314.05));
        let result = lex("314.05".to_string());
        assert_eq!(result, expected)
    }
}
