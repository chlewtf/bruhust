// Bruhust AST

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    Null,
    Custom(String),
    Array(Box<Type>),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    // lowkey x = expr  (let)
    Let { name: String, ty: Option<Type>, value: Expr, mutable: bool },
    // bussin name(params) -> ty { body }
    FnDef { name: String, params: Vec<(String, Type)>, ret_ty: Option<Type>, body: Vec<Stmt> },
    // rizz expr
    Return(Option<Expr>),
    // yeet expr
    Print(Expr),
    // vibe cond { } orcap { }
    If { cond: Expr, then: Vec<Stmt>, else_: Option<Vec<Stmt>> },
    // vibecheck cond { }
    While { cond: Expr, body: Vec<Stmt> },
    // slay i in iter { }
    For { var: String, iter: Expr, body: Vec<Stmt> },
    // based expr { arm => body, ... }
    Match { expr: Expr, arms: Vec<(MatchPattern, Vec<Stmt>)> },
    // expr statement
    Expr(Expr),
    // understood (break)
    Break,
    // periodt (continue)
    Continue,
    // oof "msg"
    Panic(Expr),
    // sheesh Name { fields }
    StructDef { name: String, fields: Vec<(String, Type)> },
}

#[derive(Debug, Clone)]
pub enum MatchPattern {
    Literal(Expr),
    Ident(String),
    Wildcard,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
    Ident(String),
    BinOp { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    UnOp { op: UnOp, expr: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Index { obj: Box<Expr>, idx: Box<Expr> },
    Field { obj: Box<Expr>, field: String },
    Array(Vec<Expr>),
    Assign { target: Box<Expr>, value: Box<Expr> },
    Cast { expr: Box<Expr>, ty: Type },
    Range { start: Box<Expr>, end: Box<Expr> },
    StructInit { name: String, fields: Vec<(String, Expr)> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Gt, Le, Ge,
    And, Or,
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Neg,
    Not,
}
