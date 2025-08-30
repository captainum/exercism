#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn answer(command: &str) -> Option<i32> {
    let tokens = command.split_whitespace().collect::<Vec<&str>>();

    if tokens.len() < 3 || tokens[0] != "What" || tokens[1] != "is" ||
        tokens.last().unwrap().chars().last().unwrap() != '?' {
        return None;
    }

    let mut result = 0;

    let mut last_op: Option<Operation> = None;

    let mut last_is_op = true;

    let mut idx: usize = 2;

    while idx < tokens.len() {
        let mut token = tokens[idx].to_string();

        if idx == tokens.len() - 1 {
            token.truncate(token.len() - 1);
        }

        if last_is_op {
            let value = token.parse::<i32>().ok()?;

            match last_op {
                Some(Operation::Add) => result += value,
                Some(Operation::Subtract) => result -= value,
                Some(Operation::Multiply) => result *= value,
                Some(Operation::Divide) => result /= value,
                None => result = value,
            }

            last_is_op = false;
        } else {
            match token.as_str() {
                "multiplied" | "divided" => {
                    if idx + 1 < tokens.len() {
                        idx += 1;
                        if tokens[idx] == "by" {
                            token = token + " by";
                        }
                    }
                },
                _ => {},
            }

            match token.as_str() {
                "plus" => last_op = Some(Operation::Add),
                "minus" => last_op = Some(Operation::Subtract),
                "multiplied by" => last_op = Some(Operation::Multiply),
                "divided by" => last_op = Some(Operation::Divide),
                _ => return None,
            }
            
            last_is_op = true
        }

        idx += 1;
    }
    
    if last_is_op {
        None
    } else {
        Some(result)
    }
}