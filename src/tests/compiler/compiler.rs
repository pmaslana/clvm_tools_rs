use std::collections::HashMap;
use std::rc::Rc;

use clvm_rs::allocator::Allocator;

use crate::classic::clvm_tools::stages::stage_0::DefaultProgramRunner;
use crate::compiler::clvm::run;
use crate::compiler::compiler::{
    DefaultCompilerOpts,
    compile_file
};
use crate::compiler::comptypes::{
    CompileErr
};
use crate::compiler::runtypes::RunFailure;
use crate::compiler::sexp::{
    SExp,
    parse_sexp
};
use crate::compiler::srcloc::Srcloc;

fn compile_string(content: &String) -> Result<String, CompileErr> {
    let mut allocator = Allocator::new();
    let runner = Rc::new(DefaultProgramRunner::new());
    let opts = Rc::new(DefaultCompilerOpts::new(&"*test*".to_string()));

    compile_file(
        &mut allocator,
        runner,
        opts,
        &content
    ).map(|x| x.to_string())
}

fn run_string(content: &String, args: &String) -> Result<Rc<SExp>, CompileErr> {
    let mut allocator = Allocator::new();
    let runner = Rc::new(DefaultProgramRunner::new());
    let srcloc = Srcloc::start(&"*test*".to_string());
    let opts = Rc::new(DefaultCompilerOpts::new(&"*test*".to_string()));
    let sexp_args = parse_sexp(srcloc.clone(), &args).map_err(|e| {
        CompileErr(e.0, e.1)
    })?[0].clone();

    compile_file(
        &mut allocator,
        runner.clone(),
        opts,
        &content
    ).and_then(|x| {
        run(
            &mut allocator,
            runner,
            Rc::new(HashMap::new()),
            Rc::new(x),
            sexp_args
        ).map_err(|e| match e {
            RunFailure::RunErr(l,s) => CompileErr(l, s),
            RunFailure::RunExn(l,s) => CompileErr(l, s.to_string())
        })
    })
}

#[test]
fn compile_test_1() {
    let result =
        compile_string(
            &"(mod () (defmacro testmacro (A) (qq (+ 1 (unquote A)))) (testmacro 3))".to_string()
        ).unwrap();
    assert_eq!(result, "(2 (1 16 (1 . 1) (1 . 3)) (4 (1) 1))".to_string());
}

#[test]
fn compile_test_2() {
    let result =
        compile_string(
            &"(mod () (defmacro if (A B C) (qq (a (i (unquote A) (com (unquote B)) (com (unquote C))) @))) (if () (+ 1 3) (+ 5 8)))".to_string()
        ).unwrap();
    assert_eq!(result, "(2 (1 2 (3 (1) (1 2 (1 16 (1 . 1) (1 . 3)) 1) (1 2 (1 16 (1 . 5) (1 . 8)) 1)) 1) (4 (1) 1))".to_string());
}

#[test]
fn compile_test_3() {
    let result =
        compile_string(
            &"(mod (A) (include *standard-cl-21*) A)".to_string()
        ).unwrap();
    assert_eq!(result, "(2 (1 . 5) (4 (1) 1))".to_string());
}

#[test]
fn compile_test_4() {
    let result =
        compile_string(
            &"(mod () (defmacro if (A B C) (qq (a (i (unquote A) (com (unquote B)) (com (unquote C))) @))) (if () (+ 1 3) (+ 5 8)))".to_string()
        ).unwrap();
    assert_eq!(result, "(2 (1 2 (3 (1) (1 2 (1 16 (1 . 1) (1 . 3)) 1) (1 2 (1 16 (1 . 5) (1 . 8)) 1)) 1) (4 (1) 1))".to_string());
}

#[test]
fn compile_test_5() {
    let result =
        compile_string(
            &"(mod (X) (include *standard-cl-21*) (defmacro testmacro (x) (qq (+ 1 (unquote x)))) (if X (testmacro 3) (testmacro 4)))".to_string()
        ).unwrap();
    assert_eq!(result, "(2 (1 2 (3 5 (1 2 (1 16 (1 . 1) (1 . 3)) 1) (1 2 (1 16 (1 . 1) (1 . 4)) 1)) 1) (4 (1) 1))".to_string());
}

#[test]
fn compile_test_6() {
    let result =
        compile_string(
            &"(mod () (list 1 2 3))".to_string()
        ).unwrap();
    assert_eq!(result, "(2 (1 4 (1 . 1) (4 (1 . 2) (4 (1 . 3) (1)))) (4 (1) 1))".to_string());
}

#[test]
fn run_test_1() {
    let result =
        run_string(
            &"(mod () (defun f (a b) (+ (* a a) b)) (f 3 1))".to_string(),
            &"()".to_string()
        ).unwrap();
    assert_eq!(result.to_string(), "10".to_string());
}

#[test]
fn run_test_2() {
    let result =
        run_string(
            &"(mod (c) (defun f (a b) (+ (* a a) b)) (f 3 c))".to_string(),
            &"(4)".to_string()
        ).unwrap();
    assert_eq!(result.to_string(), "13".to_string());
}

#[test]
fn run_test_3() {
    let result =
        run_string(
            &"(mod (arg_one) (defun factorial (input) (if (= input 1) 1 (* (factorial (- input 1)) input))) (factorial arg_one))".to_string(),
            &"(5)".to_string()
        ).unwrap();
    assert_eq!(result.to_string(), "120".to_string());
}

#[test]
fn run_test_4() {
    let result =
        run_string(
            &"(mod () (defun makelist (a) (if a (c (q . 4) (c (f a) (c (makelist (r a)) (q . ())))) (q . ()))) (makelist (q . (1 2 3))))".to_string(),
            &"()".to_string()
        ).unwrap();
    assert_eq!(result.to_string(), "(4 1 (4 2 (4 3 ())))".to_string());
}

#[test]
fn run_test_5() {
    let result =
        run_string(
            &"(mod (a) (list 1 2))".to_string(),
            &"()".to_string()
        ).unwrap();
    assert_eq!(result.to_string(), "(1 2)".to_string());
}

#[test]
fn run_test_6() {
    let result =
        run_string(
            &"(mod args (defmacro square (input) (qq (* (unquote input) (unquote input)))) (defun sqre_list (my_list) (if my_list (c (square (f my_list)) (sqre_list (r my_list))) my_list)) (sqre_list args))".to_string(),
            &"(10 9 8 7)".to_string()
        ).unwrap();
    assert_eq!(result.to_string(), "(100 81 64 49)".to_string());
}

#[test]
fn run_test_7() {
    let result =
        run_string(
            &"(mod (PASSWORD_HASH password new_puzhash amount) (defconstant CREATE_COIN 51) (defun check_password (PASSWORD_HASH password new_puzhash amount) (if (= (sha256 password) PASSWORD_HASH) (list (list CREATE_COIN new_puzhash amount)) (x))) (check_password PASSWORD_HASH password new_puzhash amount))".to_string(),
            &"(0x2ac6aecf15ac3042db34af4863da46111da7e1bf238fc13da1094f7edc8972a1 \"sha256ftw\" 0x12345678 1000000000)".to_string()
        ).unwrap();
    assert_eq!(result.to_string(), "((51 305419896 1000000000))".to_string());
}

#[test]
fn run_test_8() {
    let result =
        run_string(
            &"(mod (a b) (let ((x (+ a 1)) (y (+ b 1))) (+ x y)))".to_string(),
            &"(5 8)".to_string()
        ).unwrap();
    assert_eq!(result.to_string(), "15".to_string());
}
