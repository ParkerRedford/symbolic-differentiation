use std::{fmt::format, iter};

pub enum Expr {
    Constant(f64),
    Variable(String),

    Addition(Vec<Expr>),
    Multiply(Vec<Expr>),

    Subtract(Vec<Expr>),
    Divide(Vec<Expr>),
    Pow(Vec<Expr>),

    Neg(Box<Expr>)
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Expr::*;
        match self {
            Constant(c) => write!(f, "{}", c),
            Variable(v) => write!(f, "{}", v),
            Addition(terms) => {
                let mut first = true;
                for term in terms {
                    if !first {
                        write!(f, " + ")?;
                    }

                    write!(f, "{}", term)?;
                    first = false;
                }

                Ok(())
            },
            Subtract(terms) => {
                if terms.is_empty() {
                    return write!(f, "0");
                }

                let mut terms_iter = terms.iter();
                if let Some(first) = terms_iter.next() {
                    write!(f, "{}", first)?;
                }

                for term in terms_iter {
                    let needs_parens = matches!(term, Expr::Addition(_) | Expr::Subtract(_) | Expr::Neg(_) | Expr::Multiply(_) | Expr::Divide(_) | Expr::Pow(_));
                    if needs_parens {
                        write!(f, " - ({})", term)?;
                    } else {
                        write!(f, " - {}", term)?;
                    }
                }
                
                Ok(())
            },
            Multiply(terms) => {
                let mut first = true;
                for term in terms {
                    if !first {
                        write!(f, " * ")?;
                    }

                    let needs_parens = matches!(term, Expr::Addition(_) | Expr::Subtract(_) | Expr::Neg(_));
                    if needs_parens {
                        write!(f, "({})", term)?;
                    } else {
                        write!(f, "{}", term)?;
                    }

                    first = false;
                }
                Ok(())
            },
            Divide(terms) => {
                let s: Vec<String> = terms.iter().map(|t| format!("{}", t)).collect();
                write!(f, "({})", s.join(" / "))
            },
            Pow(terms) => {
                let s: Vec<String> = terms.iter().map(|t| format!("{}", t)).collect();
                write!(f, "({})", s.join(" ^ "))
            },
            Neg(expr) => {
                let needs_parens = matches!(**expr, Expr::Addition(_) | Expr::Subtract(_) | Expr::Neg(_));
                if needs_parens {
                    write!(f, "-({})", expr)
                } else {
                    write!(f, "{}", expr)
                }
            }                
        }
    }
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Constant(val) => *val,
            Expr::Addition(terms) => terms.iter().map(|t| t.eval()).sum(),
            Expr::Subtract(terms) => { 
                if terms.is_empty() {
                    return 0.0;
                }

                let mut iter = terms.iter();
                let first = iter.next().unwrap().eval();
                let rest_sum: f64 = iter.map(|e| -e.eval()).sum();

                first + rest_sum
             },
            Expr::Multiply(terms) => terms.iter().map(|t| t.eval()).product(),
            Expr::Divide(terms) => {
                if terms.is_empty() {
                    1.0
                } else {
                    let first = terms[0].eval();
                    let rest = terms[1..]
                        .iter()
                        .map(|t| t.eval())
                        .fold(1.0, |acc, x| acc * x);
                    
                    first / rest
                }
            },
            Expr::Pow(terms) => {
                if terms.is_empty() {
                    1.0
                } else {
                    terms[1..]
                        .iter()
                        .fold(terms[0].eval(), |acc, e| acc.powf(e.eval()))
                }
            },
            Expr::Neg(inner) => -inner.eval(),
            Expr::Variable(_) => panic!("Can't evaluate variable directly")
        }
    }

    // pub fn differentiate(&self, expr: Expr, respect_to: &str) -> Expr {
    //     match self {
    //         Expr::Constant(val) => 0,
    //         Expr::Addition(lhs, rhs) => ,
    //         Expr::Subtract(lhs, rhs) => ,
    //         Expr::Multiply(lhs, rhs) => ,
    //         Expr::Divide(lhs, rhs) => ,
    //         Expr::Pow(lhs, rhs) => ,
    //         Expr::Neg(inner) => -&inner.eval()
    //     }
    // }
}