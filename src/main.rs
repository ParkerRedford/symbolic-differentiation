pub mod expr;
use expr::*;

fn main() {
    let expr = add(vec![
        c(3), c(9), c(4),
        mul(vec![c(2), c(3), c(4), c(5)]),
        c(5 + 9), c(7), mul(vec![c(6), c(2)])
    ]);

    println!("{}", expr);
    println!("{}", expr.eval());
}
