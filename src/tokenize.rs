use crate::{Token, TokenKind};
use unicode_width::UnicodeWidthStr;

fn error_at(loc: String, input: String, error: String) -> String {
    String::from(format!(
        "{}\n{}",
        input,
        format!(
            "{}^ {}",
            (1..loc.width()).map(|_| " ").collect::<String>(),
            error
        )
    ))
}

impl Token {
    pub fn new(kind: TokenKind, str: impl Into<String>) -> Self {
        let tok = Self {
            kind,
            str: str.into(),
        };
        tok
    }

    pub fn tokenize(p: String) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];
        let chars = p.chars();
        let chars_vec = p.chars().collect::<Vec<char>>();
        let mut chars_iter = chars.clone().enumerate();

        while let Some((i, p)) = chars_iter.next() {
            log::debug!("tokens={:?}", tokens);
            if p.is_whitespace() {
                continue;
            }

            if is_ident(p) {
                let mut ident = p.to_string();
                if let Some(next_c) = chars_vec.get(i + 1) {
                    if !(is_ident(*next_c) || is_number(*next_c)) {
                        tokens.push(Self::new(TokenKind::Ident, ident));
                        continue;
                    }
                }
                while let Some((i, c)) = chars_iter.next() {
                    log::debug!("char={}", c);
                    ident.push(c);
                    if let Some(next_c) = chars_vec.get(i + 1) {
                        if !(is_ident(*next_c) || is_number(*next_c)) {
                            break;
                        }
                    }
                }
                tokens.push(Self::new(TokenKind::Ident, ident));
                continue;
            }

            if is_op(p) {
                let mut op = p.to_string();
                if let Some(next_c) = chars_vec.get(i + 1) {
                    if is_cmp_op(format!("{}{}", op, next_c)) {
                        chars_iter.next();
                        op.push(*next_c)
                    };
                }
                tokens.push(Self::new(TokenKind::Reserved, op));
                continue;
            }

            if p.is_digit(10) {
                let mut number = vec![p];
                if let Some(next_c) = chars_vec.get(i + 1) {
                    if !next_c.is_digit(10) {
                        tokens.push(Self::new(
                            TokenKind::Num(
                                number
                                    .iter()
                                    .collect::<String>()
                                    .parse::<u16>()
                                    .or_else(|_| {
                                        Err(format!("cannot convert char to integer: {:?}", number))
                                    })?,
                            ),
                            p,
                        ));
                        continue;
                    }
                }
                while let Some((i, c)) = chars_iter.next() {
                    number.push(c);
                    if let Some(next_c) = chars_vec.get(i + 1) {
                        if !next_c.is_digit(10) {
                            break;
                        }
                    }
                }
                tokens.push(Self::new(
                    TokenKind::Num(number.iter().collect::<String>().parse::<u16>().or_else(
                        |_| Err(format!("cannot convert char to integer: {:?}", number)),
                    )?),
                    p,
                ));
                continue;
            };
            return Err(error_at(
                chars
                    .clone()
                    .enumerate()
                    .filter(|(idx, _)| idx <= &i)
                    .map(|(_, v)| v)
                    .collect(),
                chars.clone().collect::<String>(),
                "cannot tokenize".to_string(),
            ));
        }

        tokens.push(Self::new(TokenKind::Eof, ""));
        Ok(tokens)
    }
}

fn is_op(ch: char) -> bool {
    ch == '+'
        || ch == '-'
        || ch == '*'
        || ch == '/'
        || ch == '('
        || ch == ')'
        || ch == ';'
        || ch == '>'
        || ch == '<'
        || ch == '='
        || ch == '!'
}

fn is_cmp_op(op: String) -> bool {
    op == "==" || op == "!=" || op == "<=" || op == ">="
}

fn is_ident(ch: char) -> bool {
    ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
}

fn is_number(ch: char) -> bool {
    ('0'..='9').contains(&ch)
}