use logos::Logos;
use scrapile::{lang::{error::Reportable, parser, targets, token::Token, typed}, scratch::{add_console, Assembly, Expr, Procedure, Statement}};

fn test_scratch() {
    let json = scrapile::scratch::assemble(&Assembly {
        stmts: vec![
            Statement::PushList { ident: "console".to_string(), value: Expr::String("hello, world!".to_string()) },
            Statement::SetVar { ident: "myvar".to_string(), value: Expr::PosInteger(49) },
            Statement::PushList { ident: "console".to_string(), value: Expr::String("that's pretty cool".to_string()) },
            Statement::PushList { ident: "mylist".to_string(), value: Expr::Float(128.0) },
            Statement::PushList { ident: "console".to_string(), value: Expr::Variable { ident: "myvar".to_string() } },
            Statement::PushList { ident: "console".to_string(), value: Expr::ListLength { ident: "mylist".to_string() } },
            Statement::PushList { ident: "console".to_string(), value: Expr::ListElement { ident: "mylist".to_string(), idx: Box::new(Expr::Integer(1)) } },

            Statement::PushList { ident: "console".to_string(), value: Expr::Add(Box::new(Expr::String("1nice".to_string())), Box::new(Expr::String("6".to_string()))) },

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

fn throw_lang_error<T>(src: &str, errors: &[impl Reportable]) -> T {
    for error in errors {
        error.report("<testing>", src);
    }

    panic!("an error occured");
}

fn test_lang() {
    // let src = r##"
    //     /*
    //         Here is a demonstration of a
    //         multi-line comment
    //     */

    //     /// The main procedure of this program
    //     main {
    //         # demonstration of an example expression
    //         println!(1 + 2 * num1 == ((4 + num2.val)) * 6 / (1, 2, "hi",) && maths.powf(1.2f, 2.6f) || version!);
    //         println!("hello, " <> "world!" + nice.say("hello") / person.(nice).file);

    //         // a nested block
    //         {
    //             println!({1 + 2; (3 * 4, "cool",)});
    //         }
    //     }
    // "##;

    // let mut tokens = Token::lexer(&src).spanned();
    // let parsed = match parser::root::parse_root(&mut tokens) {
    //     Ok(ok) => ok,
    //     Err(err) => throw_lang_error(src, &err),
    // };

    let src = r##"
        main {
            /*
                Multi-line
                Comments

                stuff without `println!` at the start get filtered out and don't get included in the scratch binary
            */
        
            println!("hello, world!");
            // some comments
            println!("THIS IS SOOO COOOL!!!!!!!");
            12;
            # some different kinds of comments
            "string";
            println!("YOOOOOOOOOOOOOO");
            "ignored"
        }
    "##;
    let mut tokens = Token::lexer(src).spanned();
    let roots = match parser::root::parse_root(&mut tokens) {
        Ok(ok) => ok,
        Err(err) => throw_lang_error(src, &err),
    };
    println!("roots: {roots:?}\n");
    let project = match typed::root::wrap_root(&roots) {
        Ok(ok) => ok,
        Err(err) => throw_lang_error(src, &[err]),
    };
    println!("project: {project:?}\n");
    let assembly = targets::scratch::translate(project);
    println!("assembly: {assembly:?}");

    let json = add_console("console", scrapile::scratch::assemble(&assembly));
    scrapile::scratch::write_to_zip("project.sb3", json).unwrap();
}

fn main() {
    test_scratch();
    test_lang();
}
