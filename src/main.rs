mod infix2onp;

fn main() {
    let infix_exp = String::from("((2+7)/3+(-14-3)*4)/2");
    infix2onp::transform_infix_to_onp(infix_exp);
    infix2onp::transform_infix_to_onp(String::from("x ^ y / (5 * z) + 10"));
}