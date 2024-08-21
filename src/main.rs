use ketchup::error::KError;
use logos::Logos;
use scrapile::{lang::{error::{parser::Error, Reportable}, parser, token::Token}, scratch::{add_console, Expr, Procedure, Statement}};

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

fn throw_lang_error<T>(src: &str, errors: &[KError<Error>]) -> T {
    for error in errors {
        error.report("<testing>", src);
    }

    panic!("an error occured");
}

fn test_lang() {
    let src = r##"
        /*
            Here is a demonstration of a
            multi-line comment
        */

        /// The main procedure of this program
        main {
            # demonstration of an example expression
            println!(1 + 2 * num1 == ((4 + num2.val)) * 6 / (1, 2, "hi",) && maths.powf(1.2, 2.6) || version!);
            println!("hello, " <> "world!");

            // a nested block
            {
                println!({1 + 2; (3 * 4, "cool",)});
            }
        }
    "##;

    let mut tokens = Token::lexer(&src).spanned();
    let parsed = match parser::root::parse_root(&mut tokens) {
        Ok(ok) => ok,
        Err(err) => throw_lang_error(src, &err),
    };

    println!("{parsed:?}");
}

fn main() {
    test_lang();
    test_scratch();
}
