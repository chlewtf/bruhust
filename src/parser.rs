use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Bool(bool),
    StringLit(String),
    Null,
    Ident(String),
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnOp {
        op: UnOp,
        expr: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Array(Vec<Expr>),
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
    },
    Input,
    ToNumber(Box<Expr>),
    ToString(Box<Expr>),
    Len(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr, bool), // bool = newline
    Let {
        name: String,
        value: Expr,
        mutable: bool,
    },
    Assign {
        name: String,
        value: Expr,
    },
    If {
        cond: Expr,
        then: Vec<Stmt>,
        else_: Option<Vec<Stmt>>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    FnDef {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Expr),
    Break,
    Continue,
    Expr(Expr),
    Assert(Expr),
    Push {
        array: String,
        value: Expr,
    },
    Match {
        expr: Expr,
        arms: Vec<(Option<Expr>, Vec<Stmt>)>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn peek2(&self) -> &Token {
        if self.pos + 1 < self.tokens.len() {
            &self.tokens[self.pos + 1]
        } else {
            &Token::EOF
        }
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        t
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline | Token::Semicolon) {
            self.advance();
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        let t = self.advance();
        if std::mem::discriminant(&t) == std::mem::discriminant(expected) {
            Ok(())
        } else {
            Err(format!(
                "expected {:?} but got {:?}, that's giving nothing 💀",
                expected, t
            ))
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.skip_newlines();
        self.expect(&Token::LBrace)?;
        self.skip_newlines();
        let mut stmts = Vec::new();
        while !matches!(self.peek(), Token::RBrace | Token::EOF) {
            let s = self.parse_stmt()?;
            stmts.push(s);
            self.skip_newlines();
        }
        self.expect(&Token::RBrace)?;
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();
        let stmt = match self.peek().clone() {
            Token::Yeet => {
                self.advance();
                let e = self.parse_expr()?;
                Ok(Stmt::Print(e, true))
            }
            Token::YeetRaw => {
                self.advance();
                let e = self.parse_expr()?;
                Ok(Stmt::Print(e, false))
            }
            Token::NoCap => {
                self.advance();
                let name = self.parse_ident()?;
                self.expect(&Token::Be)?;
                let val = self.parse_expr()?;
                Ok(Stmt::Let {
                    name,
                    value: val,
                    mutable: false,
                })
            }
            Token::Lowkey => {
                self.advance();
                let name = self.parse_ident()?;
                self.expect(&Token::Be)?;
                let val = self.parse_expr()?;
                Ok(Stmt::Let {
                    name,
                    value: val,
                    mutable: true,
                })
            }
            Token::HitsDiff => {
                self.advance();
                let name = self.parse_ident()?;
                self.expect(&Token::Be)?;
                let val = self.parse_expr()?;
                Ok(Stmt::Assign { name, value: val })
            }
            Token::FrFr => {
                self.advance();
                let cond = self.parse_expr()?;
                let then = self.parse_block()?;
                self.skip_newlines();
                let else_ = if matches!(self.peek(), Token::Nah) {
                    self.advance();
                    Some(self.parse_block()?)
                } else {
                    None
                };
                Ok(Stmt::If { cond, then, else_ })
            }
            Token::Slay => {
                self.advance();
                let cond = self.parse_expr()?;
                let body = self.parse_block()?;
                Ok(Stmt::While { cond, body })
            }
            Token::Sus => {
                self.advance();
                let name = self.parse_ident()?;
                self.expect(&Token::LParen)?;
                let mut params = Vec::new();
                while !matches!(self.peek(), Token::RParen | Token::EOF) {
                    params.push(self.parse_ident()?);
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(&Token::RParen)?;
                let body = self.parse_block()?;
                Ok(Stmt::FnDef { name, params, body })
            }
            Token::Bet => {
                self.advance();
                let e = self.parse_expr()?;
                Ok(Stmt::Return(e))
            }
            Token::Ghosted => {
                self.advance();
                Ok(Stmt::Break)
            }
            Token::Periodt => {
                self.advance();
                Ok(Stmt::Continue)
            }
            Token::CaughtIn4k => {
                self.advance();
                let e = self.parse_expr()?;
                Ok(Stmt::Assert(e))
            }
            Token::GlowUp => {
                self.advance();
                let name = self.parse_ident()?;
                let val = self.parse_expr()?;
                Ok(Stmt::Push {
                    array: name,
                    value: val,
                })
            }
            Token::VibeCheck => {
                self.advance();
                let expr = self.parse_expr()?;
                self.skip_newlines();
                self.expect(&Token::LBrace)?;
                self.skip_newlines();
                let mut arms = Vec::new();
                while !matches!(self.peek(), Token::RBrace | Token::EOF) {
                    let pattern = if matches!(self.peek(), Token::Cap) {
                        self.advance();
                        self.expect(&Token::Colon)?;
                        None
                    } else {
                        self.expect(&Token::Facts)?;
                        self.expect(&Token::Colon)?;
                        let p = self.parse_expr()?;
                        Some(p)
                    };
                    self.expect(&Token::Arrow)?;
                    let body = self.parse_block()?;
                    self.skip_newlines();
                    arms.push((pattern, body));
                }
                self.expect(&Token::RBrace)?;
                Ok(Stmt::Match { expr, arms })
            }
            _ => {
                let e = self.parse_expr()?;
                Ok(Stmt::Expr(e))
            }
        };
        // eat trailing newline/semicolon
        while matches!(self.peek(), Token::Newline | Token::Semicolon) {
            self.advance();
        }
        stmt
    }

    fn parse_ident(&mut self) -> Result<String, String> {
        match self.advance() {
            Token::Ident(s) => Ok(s),
            t => Err(format!("expected identifier but got {:?}", t)),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), Token::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinOp {
                op: BinOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_compare()?;
        while matches!(self.peek(), Token::And) {
            self.advance();
            let right = self.parse_compare()?;
            left = Expr::BinOp {
                op: BinOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_compare(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_add()?;
        loop {
            let op = match self.peek() {
                Token::EqEq => BinOp::Eq,
                Token::BangEq => BinOp::Ne,
                Token::Lt => BinOp::Lt,
                Token::Gt => BinOp::Gt,
                Token::LtEq => BinOp::Le,
                Token::GtEq => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_add()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_add(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_mul()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_mul(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Not => {
                self.advance();
                let e = self.parse_unary()?;
                Ok(Expr::UnOp {
                    op: UnOp::Not,
                    expr: Box::new(e),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        // Handle array indexing: expr[idx]
        while matches!(self.peek(), Token::LBracket) {
            self.advance();
            let idx = self.parse_expr()?;
            self.expect(&Token::RBracket)?;
            expr = Expr::Index {
                array: Box::new(expr),
                index: Box::new(idx),
            };
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s))
            }
            Token::Bussin => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            Token::Mid => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            Token::Understood => {
                self.advance();
                Ok(Expr::Null)
            }
            Token::Ratio => {
                self.advance();
                Ok(Expr::Input)
            }
            Token::Based => {
                self.advance();
                let e = self.parse_primary()?;
                Ok(Expr::ToNumber(Box::new(e)))
            }
            Token::Vibe => {
                self.advance();
                let e = self.parse_primary()?;
                Ok(Expr::ToString(Box::new(e)))
            }
            Token::NoThoughts => {
                self.advance();
                let e = self.parse_primary()?;
                Ok(Expr::Len(Box::new(e)))
            }
            Token::Drip => {
                self.advance();
                let arr_expr = self.parse_primary()?;
                self.expect(&Token::LBracket)?;
                let idx = self.parse_expr()?;
                self.expect(&Token::RBracket)?;
                Ok(Expr::Index {
                    array: Box::new(arr_expr),
                    index: Box::new(idx),
                })
            }
            Token::Sheesh => {
                self.advance();
                self.expect(&Token::LParen)?;
                let start = self.parse_expr()?;
                self.expect(&Token::Comma)?;
                let end = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(Expr::Range {
                    start: Box::new(start),
                    end: Box::new(end),
                })
            }
            Token::SusList => {
                self.advance();
                self.expect(&Token::LBracket)?;
                let mut items = Vec::new();
                while !matches!(self.peek(), Token::RBracket | Token::EOF) {
                    items.push(self.parse_expr()?);
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(&Token::RBracket)?;
                Ok(Expr::Array(items))
            }
            Token::LBracket => {
                self.advance();
                let mut items = Vec::new();
                while !matches!(self.peek(), Token::RBracket | Token::EOF) {
                    items.push(self.parse_expr()?);
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(&Token::RBracket)?;
                Ok(Expr::Array(items))
            }
            Token::Rizz => {
                self.advance();
                let name = self.parse_ident()?;
                self.expect(&Token::LParen)?;
                let mut args = Vec::new();
                while !matches!(self.peek(), Token::RParen | Token::EOF) {
                    args.push(self.parse_expr()?);
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(&Token::RParen)?;
                Ok(Expr::Call { name, args })
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                // bare function call: name(...)
                if matches!(self.peek(), Token::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.peek(), Token::RParen | Token::EOF) {
                        args.push(self.parse_expr()?);
                        if matches!(self.peek(), Token::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(&Token::RParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Token::LParen => {
                self.advance();
                let e = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(e)
            }
            t => Err(format!("unexpected token {:?} in expression, no cap", t)),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, String> {
    let mut parser = Parser::new(tokens);
    let mut stmts = Vec::new();
    parser.skip_newlines();
    while !matches!(parser.peek(), Token::EOF) {
        stmts.push(parser.parse_stmt()?);
        parser.skip_newlines();
    }
    Ok(stmts)
}
