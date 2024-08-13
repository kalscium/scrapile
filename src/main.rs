use scrapile::scratch::{add_console, Expr, Statement};

fn main() {
    let json = scrapile::scratch::assemble(
        &[
            Statement::PushList { ident: "console".to_string(), value: Expr::String("hello, world!".to_string()) },
            Statement::SetVar { ident: "myvar".to_string(), value: Expr::PosInteger(49) },
            Statement::PushList { ident: "console".to_string(), value: Expr::String("that's pretty cool".to_string()) },
            Statement::PushList { ident: "console".to_string(), value: Expr::Variable { ident: "myvar".to_string() } },
        ],
        &["myvar".to_string()],
        &[],
    );
    let json = add_console("console", json);
    scrapile::scratch::write_to_zip("test.sb3", json).unwrap();
}
