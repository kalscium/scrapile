use scrapile::scratch::{Expr, Statement};

fn main() {
    let json = scrapile::scratch::assemble(
        &[
            Statement::PushList { ident: "list1".to_string(), value: Expr::String("hello, world!".to_string()) },
        ],
        &[],
        &["list1".to_string()],
    );
    scrapile::scratch::write_to_zip("test.sb3", json).unwrap();
}
