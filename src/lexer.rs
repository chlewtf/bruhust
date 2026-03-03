#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Yeet,       // print
    YeetRaw,    // print no newline
    NoCap,      // let (immutable)
    Lowkey,     // var (mutable)
    HitsDiff,   // reassign
    Be,         // =
    FrFr,       // if
    Nah,        // else
    Slay,       // while
    Sus,        // fn
    Bet,        // return
    Rizz,       // call
    Ghosted,    // break
    Periodt,    // continue
    Bussin,     // true
    Mid,        // false
    Understood, // null
    Sheesh,     // range
    Ratio,      // input
    Based,      // to_number
    Vibe,       // to_string
    CaughtIn4k, // assert
    SusList,    // array literal keyword
    Drip,       // array index
    GlowUp,     // array push
    NoThoughts, // array length
    VibeCheck,  // match/switch
    Facts,      // match arm keyword
    Cap,        // default arm keyword
    And,        // &&
    Or,         // ||
    Not,        // !

    // Literals
    Number(f64),
    StringLit(String),
    Ident(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    BangEq,
    Lt,
    Gt,
    LtEq,
    GtEq,

    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Arrow, // =>
    Colon,
    Newline,
    Semicolon,

    Eof,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Skip whitespace (but not newlines)
        if chars[i] == ' ' || chars[i] == '\t' || chars[i] == '\r' {
            i += 1;
            continue;
        }

        // Comments: rent_free ...
        if i + 9 <= chars.len() && &chars[i..i + 9].iter().collect::<String>() == "rent_free" {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // Newline
        if chars[i] == '\n' {
            tokens.push(Token::Newline);
            i += 1;
            continue;
        }

        // String literal
        if chars[i] == '"' {
            i += 1;
            let mut s = String::new();
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 1;
                    match chars[i] {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        '"' => s.push('"'),
                        '\\' => s.push('\\'),
                        c => {
                            s.push('\\');
                            s.push(c);
                        }
                    }
                } else {
                    s.push(chars[i]);
                }
                i += 1;
            }
            if i >= chars.len() {
                return Err("unterminated string bestie 💀".to_string());
            }
            i += 1; // closing "
            tokens.push(Token::StringLit(s));
            continue;
        }

        // Number
        if chars[i].is_ascii_digit()
            || (chars[i] == '-' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
        {
            let start = i;
            if chars[i] == '-' {
                i += 1;
            }
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            let num_str: String = chars[start..i].iter().collect();
            let n: f64 = num_str
                .parse()
                .map_err(|_| format!("bad number: {}", num_str))?;
            tokens.push(Token::Number(n));
            continue;
        }

        // Identifiers and keywords
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let tok = match word.as_str() {
                "yeet_raw" => Token::YeetRaw,
                "yeet" => Token::Yeet,
                "no_cap" => Token::NoCap,
                "lowkey" => Token::Lowkey,
                "hits_diff" => Token::HitsDiff,
                "be" => Token::Be,
                "fr_fr" => Token::FrFr,
                "nah" => Token::Nah,
                "slay" => Token::Slay,
                "sus" => Token::Sus,
                "bet" => Token::Bet,
                "rizz" => Token::Rizz,
                "ghosted" => Token::Ghosted,
                "periodt" => Token::Periodt,
                "bussin" => Token::Bussin,
                "mid" => Token::Mid,
                "understood" => Token::Understood,
                "sheesh" => Token::Sheesh,
                "ratio" => Token::Ratio,
                "based" => Token::Based,
                "vibe" => Token::Vibe,
                "caught_in_4k" => Token::CaughtIn4k,
                "sus_list" => Token::SusList,
                "drip" => Token::Drip,
                "glow_up" => Token::GlowUp,
                "no_thoughts" => Token::NoThoughts,
                "vibe_check" => Token::VibeCheck,
                "facts" => Token::Facts,
                "cap" => Token::Cap,
                "and" => Token::And,
                "or" => Token::Or,
                "not" => Token::Not,
                _ => Token::Ident(word),
            };
            tokens.push(tok);
            continue;
        }

        // Two-char operators
        if i + 1 < chars.len() {
            let two: String = chars[i..i + 2].iter().collect();
            match two.as_str() {
                "==" => {
                    tokens.push(Token::EqEq);
                    i += 2;
                    continue;
                }
                "!=" => {
                    tokens.push(Token::BangEq);
                    i += 2;
                    continue;
                }
                "<=" => {
                    tokens.push(Token::LtEq);
                    i += 2;
                    continue;
                }
                ">=" => {
                    tokens.push(Token::GtEq);
                    i += 2;
                    continue;
                }
                "=>" => {
                    tokens.push(Token::Arrow);
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        // Single char
        let tok = match chars[i] {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '%' => Token::Percent,
            '<' => Token::Lt,
            '>' => Token::Gt,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ',' => Token::Comma,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            c => return Err(format!("unexpected character '{}' bestie", c)),
        };
        tokens.push(tok);
        i += 1;
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

// ─────────────────────────────────────────────
//  Unit tests
// ─────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn tok(src: &str) -> Vec<Token> {
        tokenize(src).expect("lexer failed")
    }

    #[test]
    fn lex_keywords() {
        let t = tok("yeet no_cap lowkey hits_diff be fr_fr nah slay sus bet rizz");
        assert!(t.contains(&Token::Yeet));
        assert!(t.contains(&Token::NoCap));
        assert!(t.contains(&Token::Lowkey));
        assert!(t.contains(&Token::HitsDiff));
        assert!(t.contains(&Token::Be));
        assert!(t.contains(&Token::FrFr));
        assert!(t.contains(&Token::Nah));
        assert!(t.contains(&Token::Slay));
        assert!(t.contains(&Token::Sus));
        assert!(t.contains(&Token::Bet));
        assert!(t.contains(&Token::Rizz));
    }

    #[test]
    fn lex_boolean_literals() {
        let t = tok("bussin mid");
        assert!(t.contains(&Token::Bussin));
        assert!(t.contains(&Token::Mid));
    }

    #[test]
    fn lex_number() {
        let t = tok("42 1.23");
        assert!(t.contains(&Token::Number(42.0)));
        assert!(t.contains(&Token::Number(1.23)));
    }

    #[test]
    fn lex_string() {
        let t = tok(r#""hello bestie""#);
        assert!(t.contains(&Token::StringLit("hello bestie".to_string())));
    }

    #[test]
    fn lex_operators() {
        let t = tok("+ - * / % == != < > <= >=");
        assert!(t.contains(&Token::Plus));
        assert!(t.contains(&Token::Minus));
        assert!(t.contains(&Token::Star));
        assert!(t.contains(&Token::Slash));
        assert!(t.contains(&Token::Percent));
        assert!(t.contains(&Token::EqEq));
        assert!(t.contains(&Token::BangEq));
        assert!(t.contains(&Token::Lt));
        assert!(t.contains(&Token::Gt));
        assert!(t.contains(&Token::LtEq));
        assert!(t.contains(&Token::GtEq));
    }

    #[test]
    fn lex_skips_comments() {
        let t = tok("yeet rent_free this whole line is a comment\nyeet");
        // Should have exactly 2 Yeet tokens and a Newline
        let yeets: Vec<_> = t.iter().filter(|x| **x == Token::Yeet).collect();
        assert_eq!(yeets.len(), 2);
    }

    #[test]
    fn lex_arrow() {
        let t = tok("=>");
        assert!(t.contains(&Token::Arrow));
    }

    #[test]
    fn lex_ident() {
        let t = tok("my_var");
        assert!(t.contains(&Token::Ident("my_var".to_string())));
    }
}
