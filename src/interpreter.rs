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

    pub fn evalline(&mut self, input: &str) -> Result<String, String> {
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
        assert_eq!(5, interpreter.evalline("cat is fat").unwrap().parse::<i32>().unwrap());
    }

    #[test]
    #[should_panic]
    fn sum_panic() {
        let mut interpreter = Interpreter::new(' ');
        interpreter.evalline("cat is very fat").unwrap().parse::<i32>().unwrap();
    }
}
