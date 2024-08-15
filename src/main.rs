use logos::Logos;
use scrapile::{lang::token::Token, scratch::{add_console, Expr, Procedure, Statement}};

fn test_scratch() {
    let json = scrapile::scratch::assemble(
        &[
            Statement::PushList { ident: "console".to_string(), value: Expr::String("hello, world!".to_string()) },
            Statement::SetVar { ident: "myvar".to_string(), value: Expr::PosInteger(49) },
            Statement::PushList { ident: "console".to_string(), value: Expr::String("that's pretty cool".to_string()) },
            Statement::PushList { ident: "mylist".to_string(), value: Expr::Number(128.0) },
            Statement::PushList { ident: "console".to_string(), value: Expr::Variable { ident: "myvar".to_string() } },
            Statement::PushList { ident: "console".to_string(), value: Expr::ListLength { ident: "mylist".to_string() } },
            Statement::PushList { ident: "console".to_string(), value: Expr::ListElement { ident: "mylist".to_string(), idx: Box::new(Expr::Integer(1)) } },

            Statement::CallProcedure { ident: "procedure".to_string() },
            Statement::CallProcedure { ident: "again".to_string() },
        ],
        &["myvar".to_string()],
        &["mylist".to_string()],
        &[
            Procedure { ident: "procedure".to_string(), body: &[
                Statement::PushList { ident: "console".to_string(), value: Expr::String("hello, world!".to_string()) },
                Statement::PushList { ident: "console".to_string(), value: Expr::ListElement { ident: "mylist".to_string(), idx: Box::new(Expr::Integer(1)) } },
            ] },
            Procedure { ident: "again".to_string(), body: &[
                Statement::PushList { ident: "console".to_string(), value: Expr::String("this procedure is called recursively".to_string()) },
                Statement::CallProcedure { ident: "again".to_string() },
            ] },
        ],
    );

    let json = add_console("console", json);
    
    scrapile::scratch::write_to_zip("test.sb3", json).unwrap();
}

fn test_lang() {
    let src = "hello there -hello_there 19there (1.2.4) \"nice\"";
    let tokens = Token::lexer(&src).spanned().map(|(token, span)| token.map_err(|e| (e, span)).unwrap()).collect::<Vec<_>>();

    dbg!(tokens);
}

fn main() {
    test_lang();
    test_scratch();
}
