use dovahkiin::integrated::lisp;
use dovahkiin::lexer::{lisp as lisp_lexer};
use dovahkiin::parser::{lisp as lisp_parser};

extern crate dovahkiin;

#[test]
pub fn lisp_integrated() {
    let interpreter = lisp::get_interpreter();
    let one_plus_one = "(+ 1u32 1u32)";
    println!("{:?}", lisp_lexer::tokenize_str(one_plus_one));
    lisp::eval_string(&interpreter, one_plus_one).unwrap();
}
