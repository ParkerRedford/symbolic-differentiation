use super::expr::Expr;

pub fn c<T: Into<f64>>(val: T) -> Expr { Expr::Constant(val.into()) }
pub fn v(name: &str) -> Expr { Expr::Variable(name.into()) }
pub fn add(mut terms: Vec<Expr>) -> Expr {
    let mut flat: Vec<Expr> = Vec::new();
    let mut c_sum = 0.0;

    for term in terms.drain(..) {
        match term {
            Expr::Constant(0.0) => {},
            Expr::Constant(c) => c_sum += c,
            Expr::Addition(nested) => flat.extend(nested),
            o => flat.push(o)
        }
    }

    if c_sum != 0.0 {
        flat.insert(0,Expr::Constant(c_sum));
    }

    match flat.len() {
         0 => Expr::Constant(0.0),
         1 => flat.into_iter().next().unwrap(),
         _ => Expr::Addition(flat)
    }
}

pub fn mul(args: Vec<Expr>) -> Expr { Expr::Multiply(args) }
pub fn sub(terms: Vec<Expr>) -> Expr { Expr::Subtract(terms) }
pub fn div(args: Vec<Expr>) -> Expr { Expr::Divide(args) }
pub fn pow(args: Vec<Expr>) -> Expr { Expr::Pow(args) }
pub fn neg(inner: Expr) -> Expr { Expr::Neg(Box::new(inner)) }