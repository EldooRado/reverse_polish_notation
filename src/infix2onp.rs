use std::collections::HashSet;

pub fn transform_infix_to_onp() {
    let mut stack = Vec::<char>::new();
    let expression = String::from("((2+7)/3+(-14-3)*4)/2");

    let mut is_number = false;
    let mut minus_before_number = false;
    let mut opened_bracket = false;
    let mut number : i32 = 0;

    if !expression.is_empty() && expression.chars().next().unwrap() == '-' {
        opened_bracket = true;
    }

    for c in expression.chars() {
        if opened_bracket  {
            opened_bracket = false;

            if c == '-'
            {
                minus_before_number = true;
                continue;
            }   
        }

        if c >= '0' && c <= '9' {
            is_number = true;
            number = 10*number + c.to_digit(10).unwrap() as i32;
            //println!("number{}",number);
            continue;
        }
        else if is_number {
            if minus_before_number {
                number = -number;
            }

            print!("{} ", number);
            number = 0;
            is_number = false;
            minus_before_number = false;
        }

        if c >= 'a' && c <= 'z' {
            print!("{} ", c);
        }
        else if c == '-' || c == '+' || c == '*' || c == '/' || c == '^' {
            let mut higher_or_equal_priority_operators : HashSet<char> = HashSet::<char>::new();
            higher_or_equal_priority_operators.insert('^');
            
            if c == '*' || c == '/' {
                higher_or_equal_priority_operators.insert('*');
                higher_or_equal_priority_operators.insert('/');
            }

            if c == '+' || c == '-' {
                higher_or_equal_priority_operators.insert('*');
                higher_or_equal_priority_operators.insert('/');
                higher_or_equal_priority_operators.insert('+');
                higher_or_equal_priority_operators.insert('-');
            }

            for i in (0 .. stack.len()).rev() {
                if higher_or_equal_priority_operators.contains(&stack[i]) {
                    print!("{} ", stack[i]);
                    stack.pop();
                }
                else {
                    break;
                }
            }

            stack.push(c);
        }
        else if c == '(' {
            opened_bracket = true;
            stack.push(c);
        }
        else if c == ')' {
            for i in (0 .. stack.len()).rev() {
                if stack[i] == '(' {
                    break;
                }
                print!("{} ", stack[i]);
                stack.pop();
            }
            stack.pop();
        }
        else if c.is_whitespace() {
            continue;
        }
        else {
            println!("\nSomething wrong!. Check the input.");
            std::process::exit(1);
        }
    }

    if is_number {
        print!("{} ", number);
    }
    for c in (0..stack.len()).rev() {
        print!("{} ", stack[c]);
    }
    println!();
}
