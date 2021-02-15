mod rpn_algorithms;

fn main() {
    println!("To rpn: x ^ y / (5 * z) + 10 => {}", rpn_algorithms::infix2rpn(String::from("x ^ y / (5 * z) + 10")));

    let infix_exp = String::from("((2^3+7)/3+(-14-3)*4)/2");
    let rpn_exp = rpn_algorithms::infix2rpn(infix_exp.clone());

    println!("Calculate rpn: {} => {}", infix_exp, rpn_algorithms::calculate_rpn(rpn_exp));
    
}