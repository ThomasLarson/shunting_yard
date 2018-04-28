#[derive(PartialEq)]
enum Associativity
{
    Left,
    Right
}

fn is_operator(c: &str) -> bool
{
    match c
    {
        "+" => true,
        "-" => true,
        "*" => true,
        "/" => true,
        "^" => true,
        "(" => true,
        ")" => true,
        _ => false
    }
}

fn operator_info(op: &str) -> (i32, Associativity)
{
    match op
    {
        "+" => (2, Associativity::Left),
        "-" => (2, Associativity::Left),
        "*" => (3, Associativity::Left),
        "/" => (3, Associativity::Left),
        "^" => (4, Associativity::Right),
        &_ => (0, Associativity::Left)
    }
}

fn main() {
    let input = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3";
    let mut output = String::new();
    let mut operator_stack: Vec<&str> = Vec::new();

    let tokens = input.split_whitespace();
    for t in tokens
    {
        //Check if the token is an i32, if so push to output.
        if t.parse::<i32>().is_ok()
        {
            output.push_str(t);
            output.push_str(" ");
        }
        else if is_operator(t)
        {
            if operator_stack.is_empty()
            {
                operator_stack.push(t);
            }
            //else if operator_info(operator_stack[operator_stack.len() - 1]).0 < operator_info(t).0
            else if operator_info(operator_stack[operator_stack.len() - 1]).0 < operator_info(t).0
            {
                operator_stack.push(t);
            }
            else if operator_info(operator_stack[operator_stack.len() - 1]).0 == operator_info(t).0
            {
                if operator_info(t).1 == Associativity::Left
                {
                    let op = match operator_stack.pop()
                                {
                                    Some(v) => v,
                                    None => return
                                };
                    output.push_str(op);
                    output.push_str(" ");

                    operator_stack.push(t);
                }
                else if operator_info(t).1 == Associativity::Right
                {
                    operator_stack.push(t);
                }
            }
            else if operator_info(t).0 == 0
            {
                if t == "("
                {
                    operator_stack.push(t);
                }
                else if t == ")"
                {
                    while operator_stack[operator_stack.len() - 1] != "("
                    {
                        let op = match operator_stack.pop()
                            {
                                Some(v) => v,
                                None => return
                            };
                    output.push_str(op);
                    output.push_str(" ");
                    }
                    //Discard matching '('
                    operator_stack.pop();

                }
            }
                                
        }
        else
        {
            println!("'{}' is not a valid number or operator.", t);
            return;
        }
    }

    while operator_stack.is_empty() == false
    {
        let op = match operator_stack.pop()
            {
                Some(v) => v,
                None => return
            };
        output.push_str(op);
        output.push_str(" ");
    }
    
    println!("{:?}", operator_stack);
    println!("{}", output);

}
