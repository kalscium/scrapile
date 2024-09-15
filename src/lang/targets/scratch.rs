use crate::{lang::typed::{expr::TExpr, root::Project, stmt::TStmt}, scratch::{Assembly, Expr, Statement}};

/// Translates a project into scratch assembly
pub fn translate(project: Project) -> Assembly {
    let mut stmts = vec![Statement::ClearList { ident: "console".to_string() }]; // first statement is to clear the console
    let mut variables = Vec::new();
    
    // translate the main procedure's statements
    for stmt in project.main.0.stmts {
        tstmt(stmt.0.0, &mut variables, &mut stmts);
    }
    if let Some(stmt) = project.main.0.tail {
        tstmt(stmt.0.0, &mut variables, &mut stmts);
    }

    Assembly {
        stmts,
        variables: Vec::new(),
        lists: Vec::new(),
        procedures: Vec::new(),
    }
}

const NIL: &str = "<nil>";

/// Translates a statement
pub fn tstmt(stmt: TStmt, variables: &mut Vec<String>, stmts: &mut Vec<Statement>) {
    match stmt {
        TStmt::Expr(expr) => {texpr(expr, variables, stmts);},
        TStmt::VarDeclare { ident, value } => {
            variables.push(ident.clone());
            let stmt = Statement::SetVar { ident, value: texpr(value.0.0, variables, stmts) };
            stmts.push(stmt);
        },
        _ => todo!(),
    };
}

/// Translates an expr
pub fn texpr(expr: TExpr, variables: &mut Vec<String>, stmts: &mut Vec<Statement>) -> Expr {
    
    use TExpr as E;
    match expr {
        // literals
        E::Number(num) => Expr::Float(num),
        E::String(str) => Expr::String(str),
        E::Nil => Expr::String(NIL.to_string()),

        // blocks
        E::Block(block) => {
            // append all of the block's statements
            for ((stmt, _), _) in block.stmts {
                tstmt(stmt, variables, stmts);
            }

            // return tail statement
            match block.tail {
                Some(((TStmt::Expr(tail), _), _)) => texpr(tail, variables, stmts),
                _ => Expr::String("<nil>".to_string()),
            }
        },

        // builtin-function calls
        E::BuiltinFnCall(call) => {
            use crate::lang::typed::builtin::TBuiltinFnCall as B;
            match *call {
                // convert the `as_str` builtin to it's scratch counterpart
                B::AsString((expr, _)) => texpr(expr, variables, stmts),

                // convert the `input` builtin to it's scratch counterpart
                B::Input((expr, _)) => {
                    let prompt = texpr(expr, variables, stmts);
                    stmts.push(Statement::Ask { prompt });
                    Expr::Answer
                },
                
                // convert the `println` builtin to it's scratch counterpart
                B::PrintLn(args) => {
                    let stmt = match args {
                        Some((expr, _)) => Statement::PushList { ident: "console".to_string(), value: texpr(expr, variables, stmts) },
                        None => Statement::PushList { ident: "console".to_string(), value: Expr::String(String::new()) },
                    };

                    stmts.push(stmt);
                    Expr::String(NIL.to_string())
                },
            }
        },

        // concat
        E::Concat(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, variables, stmts);
            let rhs = texpr(rhs, variables, stmts);

            Expr::Concat(Box::new(lhs), Box::new(rhs))
        },

        // maths single-space
        E::Pos(expr) => texpr(expr.0.0, variables, stmts),
        E::Neg(expr) => Expr::Mul(
            Box::new(texpr(expr.0.0, variables, stmts)),
            Box::new(Expr::Integer(-1)),
        ),

        // maths double-space
        E::Add(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, variables, stmts);
            let rhs = texpr(rhs, variables, stmts);

            Expr::Add(Box::new(lhs), Box::new(rhs))
        },
        E::Sub(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, variables, stmts);
            let rhs = texpr(rhs, variables, stmts);

            Expr::Sub(Box::new(lhs), Box::new(rhs))
        },
        E::Mul(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, variables, stmts);
            let rhs = texpr(rhs, variables, stmts);

            Expr::Mul(Box::new(lhs), Box::new(rhs))
        },
        E::Div(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, variables, stmts);
            let rhs = texpr(rhs, variables, stmts);

            Expr::Div(Box::new(lhs), Box::new(rhs))
        },

        _ => todo!(),
    }
}
