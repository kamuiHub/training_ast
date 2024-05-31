pub trait Expression {
    fn eval(&self) -> f32;
    fn to_string(&self) -> String;
}

pub struct BinaryExpression(pub char, pub Box<dyn Expression>, pub Box<dyn Expression>);
pub struct UnaryExpression(pub char, pub Box<dyn Expression>);
pub struct NumberExpression(pub f32);

impl Expression for BinaryExpression {
    fn eval(&self) -> f32 {
        match self.0 {
            '+' => self.1.eval() + self.2.eval(),
            '-' => self.1.eval() - self.2.eval(),
            '*' => self.1.eval() * self.2.eval(),
            '/' => self.1.eval() / self.2.eval(),
            _ => 0.0,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "BinaryEx: char: {:?}, ex1: {:?}, ex2: {:?}",
            self.0,
            self.1.to_string(),
            self.2.to_string()
        )
    }
}

impl Expression for UnaryExpression {
    fn eval(&self) -> f32 {
        match self.0 {
            '-' => -self.1.eval(),
            _ => self.1.eval(),
        }
    }
    fn to_string(&self) -> String {
        format!("UnaryEx: char: {:?}, ex1: {:?}", self.0, self.1.to_string())
    }
}

impl Expression for NumberExpression {
    fn eval(&self) -> f32 {
        self.0
    }
    fn to_string(&self) -> String {
        format!("NumEx: value: {:?}", self.0)
    }
}
