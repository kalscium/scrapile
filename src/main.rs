use scrapile::scratch::{set_console, Expr, Statement};

fn main() {
    let json = scrapile::scratch::assemble(
        &[
            Statement::PushList { ident: "console".to_string(), value: Expr::String("hello, world!".to_string()) },
            Statement::PushList { ident: "console".to_string(), value: Expr::String("that's pretty cool".to_string()) },
        ],
        &[],
        &["console".to_string()],
    );
    let json = set_console("console", json);
    scrapile::scratch::write_to_zip("test.sb3", json).unwrap();
}
