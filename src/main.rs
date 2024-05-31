mod expression;
use expression::{parser, tokenize};

fn main() {
    let result = tokenize("12 - 2");

    for token in result.iter() {
        println!("token: {:?}", token);
    }

    let exprs = parser(&result);
    for e in exprs.iter() {
        println!("result: {:?}", e.eval());
    }
}
