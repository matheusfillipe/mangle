use crate::operators::*;

struct Scope {}

pub struct Interpreter {
    word_separator: char,
    scope: Scope,
}

impl Interpreter {
    pub fn new(word_separator: char) -> Interpreter {
        Interpreter { scope: Scope {},
            word_separator,
        }
    }

    /// Continuously evaluate input until EOF
    pub fn eval(&mut self, input: &str) -> Result<String, String> {
        // TODO populate the scope with the variables
        // TODO evaluate
        let result = self.eval_line(input).unwrap_or("Error.".to_string());
        Ok(result.to_string())
    }

    /// Evaluates a line like if it was the full code but keeps the scope
    pub fn eval_line(&mut self, input: &str) -> Result<String, String> {
        // TODO populate the scope with the variables
        let inputlist = input.split(self.word_separator).collect::<Vec<&str>>();
        // OP var1 var2
        match inputlist[..] {
            [op, var1, var2, ..] => {
                match get_operator(op) {
                    Some(operator) => {
                        if operator.num_args != inputlist.len() - 1 {
                            return Err(format!("Invalid number of arguments for operator {}", operator.name));
                        }
                        let result = (operator.func)([var1, var2].map(|x| x.to_string()).to_vec());
                        match result.result {
                            Ok(value) => Ok(value.value),
                            Err(err) => Err(err),
                        }
                    },
                    None => Err(format!("Invalid operator {}", op)),
                }
            },
            _ => Err(format!("Operator not implemented: {}", input)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let mut interpreter = Interpreter::new(' ');
        assert_eq!(5, interpreter.eval_line("cat is fat").unwrap().parse::<i32>().unwrap());
    }

    #[test]
    #[should_panic]
    fn sum_panic() {
        let mut interpreter = Interpreter::new(' ');
        interpreter.eval_line("cat is very fat").unwrap().parse::<i32>().unwrap();
    }
}
