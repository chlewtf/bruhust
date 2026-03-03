use crate::parser::{BinOp, Expr, Stmt, UnOp};
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Null,
    Array(Vec<Value>),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Function { .. } => true,
        }
    }

    fn display(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::Bool(b) => {
                if *b {
                    "bussin".to_string()
                } else {
                    "mid".to_string()
                }
            }
            Value::Null => "understood".to_string(),
            Value::String(s) => s.clone(),
            Value::Array(a) => {
                let items: Vec<String> = a.iter().map(|v| v.display()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Function { params, .. } => format!("<sus fn({})>", params.join(", ")),
        }
    }

    fn eq_val(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum Signal {
    Return(Value),
    Break,
    Continue,
}

type Env = HashMap<String, (Value, bool)>; // (value, mutable)

pub struct Interpreter {
    globals: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        let mut env = HashMap::new();
        self.exec_block(stmts, &mut env, &mut self.globals.clone())?;
        Ok(())
    }

    fn exec_block(
        &self,
        stmts: &[Stmt],
        local: &mut Env,
        global: &mut Env,
    ) -> Result<Option<Signal>, String> {
        for stmt in stmts {
            if let Some(sig) = self.exec_stmt(stmt, local, global)? {
                return Ok(Some(sig));
            }
        }
        Ok(None)
    }

    fn exec_stmt(
        &self,
        stmt: &Stmt,
        local: &mut Env,
        global: &mut Env,
    ) -> Result<Option<Signal>, String> {
        match stmt {
            Stmt::Print(expr, newline) => {
                let val = self.eval(expr, local, global)?;
                if *newline {
                    println!("{}", val.display());
                } else {
                    print!("{}", val.display());
                }
                Ok(None)
            }
            Stmt::Let {
                name,
                value,
                mutable,
            } => {
                let val = self.eval(value, local, global)?;
                local.insert(name.clone(), (val, *mutable));
                Ok(None)
            }
            Stmt::Assign { name, value } => {
                let val = self.eval(value, local, global)?;
                if let Some((_, mutable)) = local.get(name) {
                    if !mutable {
                        return Err(format!(
                            "bestie '{}' is no_cap (immutable), can't hits_diff it 💀",
                            name
                        ));
                    }
                    local.insert(name.clone(), (val, true));
                } else if let Some((_, mutable)) = global.get(name) {
                    if !mutable {
                        return Err(format!(
                            "bestie '{}' is no_cap (immutable), can't hits_diff it 💀",
                            name
                        ));
                    }
                    global.insert(name.clone(), (val, true));
                } else {
                    return Err(format!(
                        "variable '{}' is giving ghost energy - not defined 👻",
                        name
                    ));
                }
                Ok(None)
            }
            Stmt::If { cond, then, else_ } => {
                let c = self.eval(cond, local, global)?;
                if c.is_truthy() {
                    let mut inner = local.clone();
                    if let Some(sig) = self.exec_block(then, &mut inner, global)? {
                        return Ok(Some(sig));
                    }
                } else if let Some(else_body) = else_ {
                    let mut inner = local.clone();
                    if let Some(sig) = self.exec_block(else_body, &mut inner, global)? {
                        return Ok(Some(sig));
                    }
                }
                Ok(None)
            }
            Stmt::While { cond, body } => {
                loop {
                    let c = self.eval(cond, local, global)?;
                    if !c.is_truthy() {
                        break;
                    }
                    let mut inner = local.clone();
                    let signal = self.exec_block(body, &mut inner, global)?;

                    // propagate mutable vars back
                    for (k, v) in &inner {
                        if v.1 && local.contains_key(k) {
                            local.insert(k.clone(), v.clone());
                        }
                    }

                    match signal {
                        Some(Signal::Break) => break,
                        Some(Signal::Continue) => continue,
                        Some(sig) => return Ok(Some(sig)),
                        None => {}
                    }
                }
                Ok(None)
            }
            Stmt::FnDef { name, params, body } => {
                let f = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                };
                global.insert(name.clone(), (f, false));
                Ok(None)
            }
            Stmt::Return(expr) => {
                let val = self.eval(expr, local, global)?;
                Ok(Some(Signal::Return(val)))
            }
            Stmt::Break => Ok(Some(Signal::Break)),
            Stmt::Continue => Ok(Some(Signal::Continue)),
            Stmt::Expr(e) => {
                self.eval(e, local, global)?;
                Ok(None)
            }
            Stmt::Assert(e) => {
                let val = self.eval(e, local, global)?;
                if !val.is_truthy() {
                    return Err("caught_in_4k: assertion failed, you been exposed 🚨".to_string());
                }
                Ok(None)
            }
            Stmt::Push { array, value } => {
                let val = self.eval(value, local, global)?;
                let entry = local
                    .get_mut(array)
                    .or_else(|| global.get_mut(array))
                    .ok_or_else(|| format!("array '{}' not found bestie", array))?;
                if let Value::Array(ref mut arr) = entry.0 {
                    arr.push(val);
                } else {
                    return Err(format!("'{}' ain't an array, that's mid 💀", array));
                }
                Ok(None)
            }
            Stmt::Match { expr, arms } => {
                let val = self.eval(expr, local, global)?;
                for (pattern, body) in arms {
                    let matched = match pattern {
                        None => true, // cap: (default)
                        Some(p) => {
                            let pv = self.eval(p, local, global)?;
                            val.eq_val(&pv)
                        }
                    };
                    if matched {
                        let mut inner = local.clone();
                        if let Some(sig) = self.exec_block(body, &mut inner, global)? {
                            return Ok(Some(sig));
                        }
                        break;
                    }
                }
                Ok(None)
            }
        }
    }

    fn eval(&self, expr: &Expr, local: &Env, global: &mut Env) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::StringLit(s) => Ok(Value::String(s.clone())),
            Expr::Null => Ok(Value::Null),
            Expr::Input => {
                let stdin = io::stdin();
                let mut line = String::new();
                stdin
                    .lock()
                    .read_line(&mut line)
                    .map_err(|e| format!("ratio failed: {}", e))?;
                Ok(Value::String(line.trim_end_matches('\n').to_string()))
            }
            Expr::ToNumber(e) => {
                let v = self.eval(e, local, global)?;
                match v {
                    Value::Number(n) => Ok(Value::Number(n)),
                    Value::String(s) => {
                        let n: f64 = s.trim().parse().map_err(|_| {
                            format!("based conversion failed: '{}' ain't a number fr 💀", s)
                        })?;
                        Ok(Value::Number(n))
                    }
                    Value::Bool(b) => Ok(Value::Number(if b { 1.0 } else { 0.0 })),
                    _ => Err("based conversion failed no cap 💀".to_string()),
                }
            }
            Expr::ToString(e) => {
                let v = self.eval(e, local, global)?;
                Ok(Value::String(v.display()))
            }
            Expr::Len(e) => {
                let v = self.eval(e, local, global)?;
                match v {
                    Value::Array(a) => Ok(Value::Number(a.len() as f64)),
                    Value::String(s) => Ok(Value::Number(s.len() as f64)),
                    _ => Err("no_thoughts only works on arrays and strings bestie".to_string()),
                }
            }
            Expr::Ident(name) => {
                if let Some((val, _)) = local.get(name) {
                    Ok(val.clone())
                } else if let Some((val, _)) = global.get(name) {
                    Ok(val.clone())
                } else {
                    Err(format!(
                        "'{}' is giving ghost energy - not defined 👻",
                        name
                    ))
                }
            }
            Expr::Array(items) => {
                let mut arr = Vec::new();
                for item in items {
                    arr.push(self.eval(item, local, global)?);
                }
                Ok(Value::Array(arr))
            }
            Expr::Index { array, index } => {
                let arr = self.eval(array, local, global)?;
                let idx = self.eval(index, local, global)?;
                match (arr, idx) {
                    (Value::Array(a), Value::Number(i)) => {
                        let i = i as usize;
                        a.get(i)
                            .cloned()
                            .ok_or_else(|| format!("index {} is out of bounds bestie 💀", i))
                    }
                    (Value::String(s), Value::Number(i)) => {
                        let i = i as usize;
                        s.chars()
                            .nth(i)
                            .map(|c| Value::String(c.to_string()))
                            .ok_or_else(|| format!("index {} out of bounds for string 💀", i))
                    }
                    _ => Err("indexing only works on arrays and strings bestie".to_string()),
                }
            }
            Expr::Range { start, end } => {
                let s = self.eval(start, local, global)?;
                let e = self.eval(end, local, global)?;
                match (s, e) {
                    (Value::Number(a), Value::Number(b)) => {
                        let arr: Vec<Value> = ((a as i64)..(b as i64))
                            .map(|n| Value::Number(n as f64))
                            .collect();
                        Ok(Value::Array(arr))
                    }
                    _ => Err("sheesh needs numbers fr fr".to_string()),
                }
            }
            Expr::UnOp { op, expr } => {
                let v = self.eval(expr, local, global)?;
                match op {
                    UnOp::Not => Ok(Value::Bool(!v.is_truthy())),
                    UnOp::Neg => match v {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err("can't negate non-number, that's mid".to_string()),
                    },
                }
            }
            Expr::BinOp { op, left, right } => {
                let l = self.eval(left, local, global)?;
                let r = self.eval(right, local, global)?;
                match op {
                    BinOp::Add => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                        (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
                        (Value::String(a), Value::Number(b)) => Ok(Value::String(format!(
                            "{}{}",
                            a,
                            Value::Number(b).display()
                        ))),
                        (Value::Number(a), Value::String(b)) => Ok(Value::String(format!(
                            "{}{}",
                            Value::Number(a).display(),
                            b
                        ))),
                        (Value::Array(mut a), Value::Array(b)) => {
                            a.extend(b);
                            Ok(Value::Array(a))
                        }
                        (l, r) => Err(format!(
                            "can't add {:?} and {:?}, that's not it bestie",
                            l, r
                        )),
                    },
                    BinOp::Sub => numeric_op(l, r, |a, b| a - b),
                    BinOp::Mul => numeric_op(l, r, |a, b| a * b),
                    BinOp::Div => match (&l, &r) {
                        (Value::Number(_), Value::Number(b)) if *b == 0.0 => {
                            Err("division by zero? that's not bussin 💀".to_string())
                        }
                        _ => numeric_op(l, r, |a, b| a / b),
                    },
                    BinOp::Mod => numeric_op(l, r, |a, b| a % b),
                    BinOp::Eq => Ok(Value::Bool(l.eq_val(&r))),
                    BinOp::Ne => Ok(Value::Bool(!l.eq_val(&r))),
                    BinOp::Lt => cmp_op(l, r, |a, b| a < b),
                    BinOp::Gt => cmp_op(l, r, |a, b| a > b),
                    BinOp::Le => cmp_op(l, r, |a, b| a <= b),
                    BinOp::Ge => cmp_op(l, r, |a, b| a >= b),
                    BinOp::And => Ok(Value::Bool(l.is_truthy() && r.is_truthy())),
                    BinOp::Or => Ok(Value::Bool(l.is_truthy() || r.is_truthy())),
                }
            }
            Expr::Call { name, args } => {
                let func = if let Some((v, _)) = local.get(name) {
                    v.clone()
                } else if let Some((v, _)) = global.get(name) {
                    v.clone()
                } else {
                    return Err(format!(
                        "function '{}' not found, it's giving ghost 👻",
                        name
                    ));
                };
                match func {
                    Value::Function { params, body } => {
                        if args.len() != params.len() {
                            return Err(format!(
                                "function '{}' expected {} args but got {}, that's cap 🧢",
                                name,
                                params.len(),
                                args.len()
                            ));
                        }
                        let mut fn_env: Env = HashMap::new();
                        for (param, arg) in params.iter().zip(args.iter()) {
                            let val = self.eval(arg, local, global)?;
                            fn_env.insert(param.clone(), (val, true));
                        }
                        match self.exec_block(&body, &mut fn_env, global)? {
                            Some(Signal::Return(v)) => Ok(v),
                            _ => Ok(Value::Null),
                        }
                    }
                    _ => Err(format!("'{}' ain't a function bestie", name)),
                }
            }
        }
    }
}

fn numeric_op(l: Value, r: Value, f: impl Fn(f64, f64) -> f64) -> Result<Value, String> {
    match (l, r) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(f(a, b))),
        (l, r) => Err(format!(
            "expected numbers but got {:?} and {:?}, mid 💀",
            l, r
        )),
    }
}

fn cmp_op(l: Value, r: Value, f: impl Fn(f64, f64) -> bool) -> Result<Value, String> {
    match (l, r) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(f(a, b))),
        (Value::String(a), Value::String(b)) => Ok(Value::Bool(f(a.len() as f64, b.len() as f64))),
        (l, r) => Err(format!("can't compare {:?} and {:?}", l, r)),
    }
}

// ─────────────────────────────────────────────
//  Unit tests
// ─────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::Interpreter;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn run(src: &str) -> Result<(), String> {
        let tokens = tokenize(src).map_err(|e| e.to_string())?;
        let ast = parse(tokens).map_err(|e| e.to_string())?;
        let mut interp = Interpreter::new();
        interp.run(&ast)
    }

    #[test]
    fn test_basic_print() {
        assert!(run(r#"yeet "hello""#).is_ok());
    }

    #[test]
    fn test_let_binding() {
        assert!(run("no_cap x be 42").is_ok());
    }

    #[test]
    fn test_mutable_reassign() {
        assert!(run("lowkey x be 1\nhits_diff x be 2").is_ok());
    }

    #[test]
    fn test_immutable_reassign_fails() {
        let result = run("no_cap x be 1\nhits_diff x be 2");
        assert!(result.is_err());
    }

    #[test]
    fn test_if_else() {
        assert!(run("fr_fr bussin {\nyeet \"yes\"\n} nah {\nyeet \"no\"\n}").is_ok());
    }

    #[test]
    fn test_while_loop() {
        assert!(run("lowkey i be 0\nslay i < 3 {\nhits_diff i be i + 1\n}").is_ok());
    }

    #[test]
    fn test_function_def_and_call() {
        assert!(run("sus add(a, b) {\nbet a + b\n}\nno_cap r be rizz add(2, 3)").is_ok());
    }

    #[test]
    fn test_recursion() {
        assert!(run("sus fact(n) {\nfr_fr n <= 1 {\nbet 1\n}\nbet n * rizz fact(n - 1)\n}\nno_cap r be rizz fact(5)").is_ok());
    }

    #[test]
    fn test_array_ops() {
        assert!(run("lowkey arr be [1, 2, 3]\nglow_up arr 4\nno_cap l be no_thoughts arr").is_ok());
    }

    #[test]
    fn test_range() {
        assert!(run("no_cap r be sheesh(0, 3)").is_ok());
    }

    #[test]
    fn test_string_concat() {
        assert!(run(r#"no_cap s be "hello " + "world""#).is_ok());
    }

    #[test]
    fn test_type_cast_to_number() {
        assert!(run(r#"no_cap n be based "42""#).is_ok());
    }

    #[test]
    fn test_type_cast_to_string() {
        assert!(run("no_cap s be vibe 99").is_ok());
    }

    #[test]
    fn test_boolean_logic() {
        assert!(
            run("no_cap t be bussin and bussin\nno_cap f be mid or mid\nno_cap n be not mid")
                .is_ok()
        );
    }

    #[test]
    fn test_assert_passes() {
        assert!(run("caught_in_4k bussin").is_ok());
    }

    #[test]
    fn test_assert_fails() {
        let result = run("caught_in_4k mid");
        assert!(result.is_err());
    }

    #[test]
    fn test_match() {
        let src = "no_cap x be 2\nvibe_check x {\nfacts: 2 => {\nyeet \"two\"\n}\ncap: => {\nyeet \"other\"\n}\n}";
        assert!(run(src).is_ok());
    }

    #[test]
    fn test_undefined_variable() {
        let result = run("yeet totally_undefined_var_xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_null_value() {
        assert!(run("no_cap x be understood").is_ok());
    }

    #[test]
    fn test_break_in_loop() {
        assert!(run(
            "lowkey i be 0\nslay i < 100 {\nhits_diff i be i + 1\nfr_fr i == 3 {\nghosted\n}\n}"
        )
        .is_ok());
    }

    #[test]
    fn test_continue_in_loop() {
        assert!(run(
            "lowkey i be 0\nslay i < 5 {\nhits_diff i be i + 1\nfr_fr i == 3 {\nperiodt\n}\n}"
        )
        .is_ok());
    }
}
