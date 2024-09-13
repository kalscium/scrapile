use crate::{lang::typed::{expr::TExpr, root::Project}, scratch::{Assembly, Expr, Statement}};

/// Translates a project into scratch assembly
pub fn translate(project: Project) -> Assembly {
    let mut stmts = Vec::new();
    
    // translate the main procedure's statements
    for stmt in project.main.0.stmts {
        let _ = texpr(stmt.0.0, &mut stmts);
    }
    if let Some(stmt) = project.main.0.tail {
        let _ = texpr(stmt.0.0, &mut stmts);
    }

    Assembly {
        stmts,
        variables: Vec::new(),
        lists: Vec::new(),
        procedures: Vec::new(),
    }
}

/// Translates an expr
pub fn texpr(expr: TExpr, stmts: &mut Vec<Statement>) -> Expr {
    const NIL: &str = "<nil>";
    
    use TExpr as E;
    match expr {
        // literals
        E::Number(num) => Expr::Float(num),
        E::String(str) => Expr::String(str),
        E::Nil => Expr::String(NIL.to_string()),

        // builtin-function calls
        E::BuiltinFnCall(call) => {
            use crate::lang::typed::builtin::TBuiltinFnCall as B;
            let stmt = match *call {
                // convert the println builtin to it's scratch counterpart
                B::PrintLn(args) => match args {
                    Some(((expr, _), _)) => Statement::PushList { ident: "console".to_string(), value: texpr(expr, stmts) },
                    None => Statement::PushList { ident: "console".to_string(), value: Expr::String(String::new()) },
                },
            };

            stmts.push(stmt);
            Expr::String(NIL.to_string())
        },

        // maths
        E::Add(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, stmts);
            let rhs = texpr(rhs, stmts);

            Expr::Add(Box::new(lhs), Box::new(rhs))
        },
        E::Sub(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, stmts);
            let rhs = texpr(rhs, stmts);

            Expr::Sub(Box::new(lhs), Box::new(rhs))
        },
        E::Mul(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, stmts);
            let rhs = texpr(rhs, stmts);

            Expr::Mul(Box::new(lhs), Box::new(rhs))
        },
        E::Div(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, stmts);
            let rhs = texpr(rhs, stmts);

            Expr::Div(Box::new(lhs), Box::new(rhs))
        },

        _ => todo!(),
    }
}
