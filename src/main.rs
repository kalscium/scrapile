use std::fs;

use logos::Logos;
use scrapile::{lang::{error::Reportable, parser, targets, token::Token, typed}, scratch::{add_console, Assembly, Condition, Expr, Procedure, Statement}};

fn test_scratch() {
    let json = scrapile::scratch::assemble(Assembly {
        stmts: vec![
            Statement::PushList { ident: "console".to_string(), value: Expr::String("hello, world!".to_string()) },
            Statement::SetVar { ident: "myvar".to_string(), value: Expr::PosInteger(49) },
            Statement::PushList { ident: "console".to_string(), value: Expr::String("that's pretty cool".to_string()) },
            Statement::PushList { ident: "mylist".to_string(), value: Expr::Float(128.0) },
            Statement::PushList { ident: "console".to_string(), value: Expr::Variable { ident: "myvar".to_string() } },
            Statement::PushList { ident: "console".to_string(), value: Expr::ListLength { ident: "mylist".to_string() } },
            Statement::PushList { ident: "console".to_string(), value: Expr::ListElement { ident: "mylist".to_string(), idx: Box::new(Expr::Integer(1)) } },

            Statement::PushList { ident: "console".to_string(), value: Expr::Add(Box::new(Expr::String("1nice".to_string())), Box::new(Expr::String("6".to_string()))) },

            Statement::IfElse {
                condition: Condition::EqualTo(Expr::String("false".to_string()), Expr::String("true".to_string())),
                body: vec![
                    Statement::PushList { ident: "console".to_string(), value:Expr::String("is true".to_string())  },
                    Statement::RepeatUntil {
                        condition: Condition::EqualTo(Expr::String("true".to_string()), Expr::String("not_equal".to_string())),
                        body: vec![
                            Statement::PushList { ident: "console".to_string(), value:Expr::String("forever".to_string())  },
                        ],
                    },
                ],
                otherwise: vec![
                    Statement::PushList { ident: "console".to_string(), value:Expr::String("is false".to_string())  },
                    Statement::If {
                        condition: Condition::EqualTo(Expr::String("true".to_string()), Expr::String("true".to_string())),
                        body: vec![
                            Statement::PushList { ident: "console".to_string(), value: Expr::String("hello, world from an if statement!".to_string()) },
                        ],
                    },
                ],
            },

            Statement::CallProcedure { ident: "procedure".to_string() },
            Statement::CallProcedure { ident: "again".to_string() },
        ],
        variables: vec![ "myvar".to_string() ],
        lists: vec![ "myvar".to_string() ],
        procedures: vec![
            Procedure { ident: "procedure".to_string(), body: vec![
                Statement::PushList { ident: "console".to_string(), value: Expr::String("hello, world!".to_string()) },
                Statement::PushList { ident: "console".to_string(), value: Expr::ListElement { ident: "mylist".to_string(), idx: Box::new(Expr::Integer(1)) } },
            ] },
            Procedure { ident: "again".to_string(), body: vec![
                Statement::PushList { ident: "console".to_string(), value: Expr::String("this procedure is called recursively".to_string()) },
                Statement::CallProcedure { ident: "again".to_string() },
            ] },
        ],
    });

    let json = add_console("console", json);
    
    scrapile::scratch::write_to_zip("test.sb3", json).unwrap();
}

fn throw_lang_error<T>(src: &str, src_id: &str, errors: &[impl Reportable]) -> T {
    for error in errors {
        error.report(src_id, src);
    }

    panic!("an error occured");
}

fn test_lang() {
    let src = fs::read_to_string(std::env::args().collect::<Vec<_>>()[1].clone()).unwrap();

    let mut tokens = Token::lexer(&src).spanned();
    let roots = match parser::root::parse_root(&mut tokens) {
        Ok(ok) => ok,
        Err(err) => throw_lang_error(&src, "example.srpl", &err),
    };
    println!("roots: {roots:?}\n");
    let project = match typed::root::wrap_root(&roots) {
        Ok(ok) => ok,
        Err(err) => throw_lang_error(&src, "example.srpl", &[err]),
    };
    println!("project: {project:?}\n");
    let assembly = targets::scratch::translate(project);
    println!("assembly: {assembly:?}");

    let json = add_console("console", scrapile::scratch::assemble(assembly));
    scrapile::scratch::write_to_zip("project.sb3", json).unwrap();
}

fn main() {
    test_scratch();
    test_lang();
}
