use std::collections::HashSet;

pub fn transform_infix_to_onp(infix_expression : String) {
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
    
    for c in infix_expression.chars() {
        match parser_state {
            ParserState::Default => {}
            ParserState::Number => {
                if c < '0' || c > '9' {
                    //change to one line if
                    if number_info.negative {
                        number_info.value = -number_info.value;
                    }
        
                    print!("{} ", number_info.value);
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
            print!("{} ", c);
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
            stack.push(c);
            parser_state = ParserState::MinusCheck;
        }
        else if c == ')' {
            //pop all operators from stack till a opening bracket
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
            //skip white characters
            continue;
        }
        else {
            println!("\nSomething wrong!. Check the input.");
            std::process::exit(1);
        }
    }

    // all characters are parsed. If last state is 'ParserState::Number' it means that
    // some number is calculated and we need print it 
    if let ParserState::Number = parser_state {
        print!("{} ", number_info.value);
    }

    // at the end we should print all operators from the stack 
    for c in (0..stack.len()).rev() {
        print!("{} ", stack[c]);
    }
    println!();
}
