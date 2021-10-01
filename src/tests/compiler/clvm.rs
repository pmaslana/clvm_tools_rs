use std::borrow::Borrow;
use std::rc::Rc;

use num_bigint::ToBigInt;

use clvm_rs::allocator::Allocator;

use crate::classic::clvm_tools::stages::stage_0::DefaultProgramRunner;

use crate::compiler::sexp::{
    SExp,
    parse_sexp
};
use crate::compiler::srcloc::Srcloc;
use crate::compiler::runtypes::RunFailure;
use crate::compiler::clvm::parse_and_run;

/*
let clvm_tests : RunExecTest.t list =
    [ { expected = RunOk "(\"there\" \"fool\")"
        ; input = "(a (q 2 4 (c 2 (c 6 ()))) (c (q 13 26729 \"there\" \"fool\") 1))"
        ; args = "()"
    }
        ; { expected = RunOk "(4 1 (4 2 ()))"
            ; input = "(a (q 2 (q 2 2 (c 2 (c 3 (q)))) (c (q 2 (i 5 (q 4 (q . 4) (c 9 (c (a 2 (c 2 (c 13 (q)))) (q)))) (q 1)) 1) 1)) 1)"
            ; args = "(1 2)"
        }
        ; { expected = RunOk "13"
            ; input = "(2 (3 (1) (1 16 (1 . 1) (1 . 3)) (1 16 (1 . 5) (1 . 8))) 1)"
            ; args = "()"
        }
        ; { expected = RunOk "(30000 . 3392)"
            ; input = "(divmod (1 . 300000003392) (1 . 10000000))"
            ; args = "()"
        }
    ]
 */

fn test_compiler_clvm(
    to_run: &String,
    args: &String
) -> Result<Rc<SExp>, RunFailure> {
    let mut allocator = Allocator::new();
    let runner = Rc::new(DefaultProgramRunner::new());
    parse_and_run(
        &mut allocator,
        runner,
        &"*test*".to_string(),
        &to_run,
        &args
    )
}

#[test]
fn test_sexp_parse_1() {
    let loc = Srcloc::start(&"*test*".to_string());
    let res = parse_sexp(loc, &"()".to_string()).map(|x| x[0].to_string());
    assert_eq!(res, Ok("()".to_string()));
}

#[test]
fn test_sexp_parse_2() {
    let loc = Srcloc::start(&"*test*".to_string());
    let res = parse_sexp(loc, &"55".to_string()).and_then(|x| x[0].get_number());
    assert_eq!(res, Ok(55_i32.to_bigint().unwrap()));
}

#[test]
fn test_sexp_parse_3() {
    let loc = Srcloc::start(&"*test*".to_string());
    let res = parse_sexp(loc, &"hello".to_string()).and_then(|x| x[0].get_number());
    assert_eq!(res, Ok(448378203247_i64.to_bigint().unwrap()));
}

#[test]
fn test_sexp_parse_4() {
    let loc = Srcloc::start(&"*test*".to_string());
    let res = parse_sexp(loc, &"\"hello\"".to_string()).and_then(|x| x[0].get_number());
    assert_eq!(res, Ok(448378203247_i64.to_bigint().unwrap()));
}

#[test]
fn test_sexp_parse_5() {
    let loc = Srcloc::start(&"*test*".to_string());
    let res = parse_sexp(loc, &"(3 . 4)".to_string()).map(|x| x[0].to_string());
    assert_eq!(res, Ok("(3 . 4)".to_string()));
}

#[test]
fn test_sexp_parse_6() {
    let loc = Srcloc::start(&"*test*".to_string());
    let res = parse_sexp(loc, &"(\" \" . 3)".to_string()).map(|x| x[0].to_string());
    assert_eq!(res, Ok("(\" \" . 3)".to_string()));
}

#[test]
fn test_sexp_parse_7() {
    let loc = Srcloc::start(&"*test*".to_string());
    let res = parse_sexp(loc, &"(3 . \" \")".to_string()).map(|x| x[0].to_string());
    assert_eq!(res, Ok("(3 . \" \")".to_string()));
}

#[test]
fn test_sexp_parse_8() {
    let loc = Srcloc::start(&"*test*".to_string());
    let res = parse_sexp(loc, &"(a (q 2 4 (c 2 (c 6 ()))) (c (q 13 26729 \"there\" \"fool\") 1))".to_string()).map(|x| x[0].to_string());
    assert_eq!(res, Ok("(a (q 2 4 (c 2 (c 6 ()))) (c (q 13 26729 \"there\" \"fool\") 1))".to_string()));
}

#[test]
fn test_clvm_1() {
    let loc = Srcloc::start(&"*test*".to_string());
    let result =
        test_compiler_clvm(
            &"(a (q 2 4 (c 2 (c 6 ()))) (c (q 13 26729 \"there\" \"fool\") 1))".to_string(),
            &"()".to_string()
        ).unwrap();
    let want =
        parse_sexp(loc, &"(\"there\" \"fool\")".to_string()).unwrap();

    print!("result {}\n", result.to_string());
    print!("want {}\n", want[0].to_string());
    assert!(result.equal_to(want[0].borrow()));
}
