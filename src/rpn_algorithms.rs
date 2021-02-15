use std::collections::HashSet;

pub fn infix2rpn(infix_expression : String) -> String{
    enum ParserState {
        Default,
        Number,
        MinusCheck
    }

    struct NumberInfo {
        value : i32,
        negative : bool
    }

    let higher_or_equal_priority_operators_than_plus_and_minus : HashSet<char> = 
        vec!['^', '/', '*', '+', '-'].into_iter().collect();

    let higher_or_equal_priority_operators_multiply_and_divide_sign : HashSet<char> = 
        vec!['^', '/', '*'].into_iter().collect();
        
    let higher_or_equal_priority_operators_than_power_sign : HashSet<char> = 
        vec!['^'].into_iter().collect();

    let mut parser_state : ParserState = ParserState::MinusCheck;
    let mut stack = Vec::<char>::new();
    let mut number_info  = NumberInfo {value: 0, negative: false};
    
    let mut rpn_string = String::new();

    for c in infix_expression.chars() {
        match parser_state {
            ParserState::Default => {}
            ParserState::Number => {
                if c < '0' || c > '9' {
                    //change to one line if
                    if number_info.negative {
                        number_info.value = -number_info.value;
                    }
        
                    rpn_string.push_str(&number_info.value.to_string());
                    rpn_string.push(' ');

                    number_info.value = 0;
                    number_info.negative = false;
    
                    parser_state = ParserState::Default;
                }
            }

            // program needs to recognize a negative numbers so minus have two means. 
            // First case when means substraction, second number is negative. Minus works
            // in second way, only when is at the begin of expression or appears immediately 
            // after opened bracket. I named this state as 'MinusCheck'
            ParserState::MinusCheck => {
                parser_state = ParserState::Default;

                if c == '-' {
                    number_info.negative = true;
                    continue;
                }
            }
        };

        if c >= '0' && c <= '9' {
            //if parser encouters a digit it means that there is a number
            parser_state = ParserState::Number;
            number_info.value = 10*number_info.value + c.to_digit(10).unwrap() as i32;
            continue;
        }
        else if c >= 'a' && c <= 'z' {
            rpn_string.push(c);
            rpn_string.push(' ');
        }
        else if c == '-' || c == '+' || c == '*' || c == '/' || c == '^' {
            // the most complicated part. We need pop all operators till we find lower priority operator 

            let mut higher_or_equal_priority_operators : &HashSet<char> = &HashSet::<char>::new();
            match c {
                '^' => {
                    higher_or_equal_priority_operators = &higher_or_equal_priority_operators_than_power_sign;
                }

                '*' | '/' => {
                    higher_or_equal_priority_operators = &higher_or_equal_priority_operators_multiply_and_divide_sign;
                }

                '+' | '-' => {
                    higher_or_equal_priority_operators = &higher_or_equal_priority_operators_than_plus_and_minus;
                }

                _ => {
                    //impossible case
                }
            }
            

            for i in (0 .. stack.len()).rev() {
                if higher_or_equal_priority_operators.contains(&stack[i]) {
                    rpn_string.push(stack[i]);
                    rpn_string.push(' ');
                    stack.pop();
                }
                else {
                    break;
                }
            }

            stack.push(c);
        }
        else if c == '(' {
            stack.push(c);
            parser_state = ParserState::MinusCheck;
        }
        else if c == ')' {
            //pop all operators from stack till a opening bracket
            for i in (0 .. stack.len()).rev() {
                if stack[i] == '(' {
                    break;
                }

                rpn_string.push(stack[i]);
                rpn_string.push(' ');
                stack.pop();
            }
            stack.pop();
        }
        else if c.is_whitespace() {
            //skip white characters
            continue;
        }
        else {
            println!("\nERROR: Input is not ifix expression.");
            std::process::exit(1);
        }
    }

    // all characters are parsed. If last state is 'ParserState::Number' it means that
    // some number is calculated and we need print it 
    if let ParserState::Number = parser_state {
        rpn_string.push_str(&number_info.value.to_string());
        rpn_string.push(' ');
    }

    // at the end we should print all operators from the stack 
    for c in (0..stack.len()).rev() {
        rpn_string.push(stack[c]);
        rpn_string.push(' ');
    }
    rpn_string
}

fn is_string_numeric(s: String) -> bool {
    
    let mut first_char = true;
    for c in s.chars() {
        if first_char && s.len() >= 2 {
            //check if first char is '-'
            first_char = false;
            if c == '-' {
                continue;
            }
        }

        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

pub fn calculate_rpn(rpn_expression : String) -> i32 {

    let mut stack = Vec::<i32>::new();
    let elements = rpn_expression.split(" ");

    for s in elements {
        if s.len() == 0 {
            continue;
        }
        else if is_string_numeric(s.to_string()) {
            stack.push(s.parse().unwrap());
            
        }
        else if s == "^" || s == "+" || s == "-" || s == "*" || s == "/" {
            if stack.len() < 2 {
                println!("\nERROR: Input is incorrect expression.");
                return 0
            }

            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            match s {
                "+" => {
                    stack.push(a + b);
                }
                "-" => {
                    stack.push(a - b);
                }
                "*" => {
                    stack.push(a * b);
                }
                "/" => {
                    stack.push(a / b);
                }
                "^" => {
                    stack.push(a.pow(b as u32));
                }
                _ => {}
            }
        }
        else {
            println!("\nERROR: Input is not arythmetic expression.");
            return 0
        }
    }
    if stack.len() > 1 {
        println!("\nERROR: Something wrong.");
        return 0
    }
    stack.pop().unwrap()
}