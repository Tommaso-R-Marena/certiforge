//! Textual CertIR surface syntax.
//!
//! ```text
//! fn name(x: u32, y: u32) -> u32 {
//!   add(and(x, y), xor(x, y))
//! }
//! ```

use certir::{BinOp, CmpOp, Expr, Param, Program, Ty, UnOp, Width};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("parse error at {line}:{col}: {message}")]
    At {
        line: usize,
        col: usize,
        message: String,
    },
    #[error("{0}")]
    Message(String),
}

#[derive(Clone, Debug)]
struct Tok {
    kind: TokKind,
    line: usize,
    col: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TokKind {
    Ident(String),
    Number(u64),
    Bool(bool),
    Fn,
    Arrow,
    Colon,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Eof,
}

struct Lexer<'a> {
    src: &'a [u8],
    i: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            i: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.i).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let c = self.peek()?;
        self.i += 1;
        if c == b'\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn skip_ws_comments(&mut self) {
        loop {
            while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
                self.bump();
            }
            if self.peek() == Some(b'/') && self.src.get(self.i + 1) == Some(&b'/') {
                while self.peek().is_some_and(|c| c != b'\n') {
                    self.bump();
                }
                continue;
            }
            break;
        }
    }

    fn next_tok(&mut self) -> Result<Tok, ParseError> {
        self.skip_ws_comments();
        let line = self.line;
        let col = self.col;
        let Some(c) = self.peek() else {
            return Ok(Tok {
                kind: TokKind::Eof,
                line,
                col,
            });
        };
        match c {
            b'(' => {
                self.bump();
                Ok(Tok {
                    kind: TokKind::LParen,
                    line,
                    col,
                })
            }
            b')' => {
                self.bump();
                Ok(Tok {
                    kind: TokKind::RParen,
                    line,
                    col,
                })
            }
            b'{' => {
                self.bump();
                Ok(Tok {
                    kind: TokKind::LBrace,
                    line,
                    col,
                })
            }
            b'}' => {
                self.bump();
                Ok(Tok {
                    kind: TokKind::RBrace,
                    line,
                    col,
                })
            }
            b',' => {
                self.bump();
                Ok(Tok {
                    kind: TokKind::Comma,
                    line,
                    col,
                })
            }
            b':' => {
                self.bump();
                Ok(Tok {
                    kind: TokKind::Colon,
                    line,
                    col,
                })
            }
            b'-' if self.src.get(self.i + 1) == Some(&b'>') => {
                self.bump();
                self.bump();
                Ok(Tok {
                    kind: TokKind::Arrow,
                    line,
                    col,
                })
            }
            b'0'..=b'9' => {
                let mut v: u64 = 0;
                while let Some(d @ b'0'..=b'9') = self.peek() {
                    self.bump();
                    v = v
                        .checked_mul(10)
                        .and_then(|x| x.checked_add((d - b'0') as u64))
                        .ok_or_else(|| ParseError::At {
                            line,
                            col,
                            message: "integer literal overflow".into(),
                        })?;
                }
                Ok(Tok {
                    kind: TokKind::Number(v),
                    line,
                    col,
                })
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let mut s = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_alphanumeric() || ch == b'_' {
                        s.push(ch as char);
                        self.bump();
                    } else {
                        break;
                    }
                }
                let kind = match s.as_str() {
                    "fn" => TokKind::Fn,
                    "true" => TokKind::Bool(true),
                    "false" => TokKind::Bool(false),
                    _ => TokKind::Ident(s),
                };
                Ok(Tok { kind, line, col })
            }
            _ => Err(ParseError::At {
                line,
                col,
                message: format!("unexpected character {:?}", c as char),
            }),
        }
    }
}

struct Parser {
    toks: Vec<Tok>,
    i: usize,
}

impl Parser {
    fn new(src: &str) -> Result<Self, ParseError> {
        let mut lx = Lexer::new(src);
        let mut toks = Vec::new();
        loop {
            let t = lx.next_tok()?;
            let done = t.kind == TokKind::Eof;
            toks.push(t);
            if done {
                break;
            }
        }
        Ok(Self { toks, i: 0 })
    }

    fn peek(&self) -> &Tok {
        &self.toks[self.i]
    }

    fn bump(&mut self) -> &Tok {
        let t = &self.toks[self.i];
        if self.i + 1 < self.toks.len() {
            self.i += 1;
        }
        t
    }

    fn expect(&mut self, kind: &TokKind) -> Result<(), ParseError> {
        let t = self.peek().clone();
        if std::mem::discriminant(&t.kind) == std::mem::discriminant(kind)
            || t.kind == *kind
        {
            // For Ident/Number we only match variant shape above; handle exactly:
            match (&t.kind, kind) {
                (TokKind::Fn, TokKind::Fn)
                | (TokKind::Arrow, TokKind::Arrow)
                | (TokKind::Colon, TokKind::Colon)
                | (TokKind::Comma, TokKind::Comma)
                | (TokKind::LParen, TokKind::LParen)
                | (TokKind::RParen, TokKind::RParen)
                | (TokKind::LBrace, TokKind::LBrace)
                | (TokKind::RBrace, TokKind::RBrace)
                | (TokKind::Eof, TokKind::Eof) => {
                    self.bump();
                    Ok(())
                }
                _ => Err(ParseError::At {
                    line: t.line,
                    col: t.col,
                    message: format!("expected {kind:?}, got {:?}", t.kind),
                }),
            }
        } else {
            Err(ParseError::At {
                line: t.line,
                col: t.col,
                message: format!("expected {kind:?}, got {:?}", t.kind),
            })
        }
    }

    fn parse_ty(&mut self) -> Result<Ty, ParseError> {
        let t = self.peek().clone();
        match &t.kind {
            TokKind::Ident(s) if s == "bool" => {
                self.bump();
                Ok(Ty::Bool)
            }
            TokKind::Ident(s) => {
                let w = Width::parse(s).ok_or_else(|| ParseError::At {
                    line: t.line,
                    col: t.col,
                    message: format!("unknown type `{s}`"),
                })?;
                self.bump();
                Ok(Ty::BitVec(w))
            }
            _ => Err(ParseError::At {
                line: t.line,
                col: t.col,
                message: "expected type".into(),
            }),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let t = self.peek().clone();
        match &t.kind {
            TokKind::Bool(b) => {
                self.bump();
                Ok(Expr::ConstBool { value: *b })
            }
            TokKind::Number(n) => {
                self.bump();
                // Bare numbers need a width suffix via constructor: u32(n)
                Err(ParseError::At {
                    line: t.line,
                    col: t.col,
                    message: format!(
                        "bare integer {n}; use width constructor e.g. u32({n})"
                    ),
                })
            }
            TokKind::Ident(name) => {
                let name = name.clone();
                self.bump();
                if self.peek().kind == TokKind::LParen {
                    self.bump();
                    // width constructor or operator application
                    if let Some(w) = Width::parse(&name) {
                        let nt = self.peek().clone();
                        let TokKind::Number(v) = nt.kind else {
                            return Err(ParseError::At {
                                line: nt.line,
                                col: nt.col,
                                message: "expected integer in width constructor".into(),
                            });
                        };
                        self.bump();
                        self.expect(&TokKind::RParen)?;
                        return Ok(Expr::ConstBv {
                            width: w,
                            value: v & w.mask(),
                        });
                    }
                    if let Some(op) = UnOp::parse(&name) {
                        let e = self.parse_expr()?;
                        self.expect(&TokKind::RParen)?;
                        return Ok(Expr::UnOp {
                            op,
                            expr: Box::new(e),
                        });
                    }
                    if let Some(op) = BinOp::parse(&name) {
                        let lhs = self.parse_expr()?;
                        self.expect(&TokKind::Comma)?;
                        let rhs = self.parse_expr()?;
                        self.expect(&TokKind::RParen)?;
                        return Ok(Expr::BinOp {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        });
                    }
                    if let Some(op) = CmpOp::parse(&name) {
                        let lhs = self.parse_expr()?;
                        self.expect(&TokKind::Comma)?;
                        let rhs = self.parse_expr()?;
                        self.expect(&TokKind::RParen)?;
                        return Ok(Expr::Cmp {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        });
                    }
                    if name == "select" {
                        let cond = self.parse_expr()?;
                        self.expect(&TokKind::Comma)?;
                        let then_expr = self.parse_expr()?;
                        self.expect(&TokKind::Comma)?;
                        let else_expr = self.parse_expr()?;
                        self.expect(&TokKind::RParen)?;
                        return Ok(Expr::Select {
                            cond: Box::new(cond),
                            then_expr: Box::new(then_expr),
                            else_expr: Box::new(else_expr),
                        });
                    }
                    return Err(ParseError::At {
                        line: t.line,
                        col: t.col,
                        message: format!("unknown operator `{name}`"),
                    });
                }
                Ok(Expr::Var { name })
            }
            _ => Err(ParseError::At {
                line: t.line,
                col: t.col,
                message: "expected expression".into(),
            }),
        }
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        self.expect(&TokKind::Fn)?;
        let name_tok = self.peek().clone();
        let TokKind::Ident(name) = &name_tok.kind else {
            return Err(ParseError::At {
                line: name_tok.line,
                col: name_tok.col,
                message: "expected function name".into(),
            });
        };
        let name = name.clone();
        self.bump();
        self.expect(&TokKind::LParen)?;
        let mut params = Vec::new();
        if self.peek().kind != TokKind::RParen {
            loop {
                let pt = self.peek().clone();
                let TokKind::Ident(pname) = &pt.kind else {
                    return Err(ParseError::At {
                        line: pt.line,
                        col: pt.col,
                        message: "expected parameter name".into(),
                    });
                };
                let pname = pname.clone();
                self.bump();
                self.expect(&TokKind::Colon)?;
                let ty = self.parse_ty()?;
                params.push(Param { name: pname, ty });
                if self.peek().kind == TokKind::Comma {
                    self.bump();
                    continue;
                }
                break;
            }
        }
        self.expect(&TokKind::RParen)?;
        self.expect(&TokKind::Arrow)?;
        let ret_ty = self.parse_ty()?;
        self.expect(&TokKind::LBrace)?;
        let body = self.parse_expr()?;
        self.expect(&TokKind::RBrace)?;
        Ok(Program {
            name,
            params,
            body,
            ret_ty,
        })
    }
}

pub fn parse_program(src: &str) -> Result<Program, ParseError> {
    let mut p = Parser::new(src)?;
    let prog = p.parse_program()?;
    if p.peek().kind != TokKind::Eof {
        let t = p.peek();
        return Err(ParseError::At {
            line: t.line,
            col: t.col,
            message: "trailing tokens".into(),
        });
    }
    Ok(prog)
}

pub fn pretty(expr: &Expr) -> String {
    match expr {
        Expr::ConstBool { value } => value.to_string(),
        Expr::ConstBv { width, value } => format!("{width}({value})"),
        Expr::Var { name } => name.clone(),
        Expr::UnOp { op, expr } => format!("{}({})", op.as_str(), pretty(expr)),
        Expr::BinOp { op, lhs, rhs } => {
            format!("{}({}, {})", op.as_str(), pretty(lhs), pretty(rhs))
        }
        Expr::Cmp { op, lhs, rhs } => {
            format!("{}({}, {})", op.as_str(), pretty(lhs), pretty(rhs))
        }
        Expr::Select {
            cond,
            then_expr,
            else_expr,
        } => format!(
            "select({}, {}, {})",
            pretty(cond),
            pretty(then_expr),
            pretty(else_expr)
        ),
    }
}

pub fn pretty_program(p: &Program) -> String {
    let params = p
        .params
        .iter()
        .map(|par| format!("{}: {}", par.name, par.ty))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "fn {}({}) -> {} {{\n  {}\n}}\n",
        p.name,
        params,
        p.ret_ty,
        pretty(&p.body)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use certir::BinOp;

    #[test]
    fn parse_or_via_add() {
        let src = r#"
        fn or_via_add(x: u32, y: u32) -> u32 {
          add(and(x, y), xor(x, y))
        }
        "#;
        let p = parse_program(src).unwrap();
        p.check().unwrap();
        assert_eq!(p.name, "or_via_add");
        match p.body {
            Expr::BinOp { op: BinOp::Add, .. } => {}
            _ => panic!("expected add"),
        }
    }

    #[test]
    fn roundtrip_pretty() {
        let src = "fn f(x: u8) -> u8 {\n  add(x, u8(1))\n}\n";
        let p = parse_program(src).unwrap();
        let out = pretty_program(&p);
        let p2 = parse_program(&out).unwrap();
        assert_eq!(p, p2);
    }
}
