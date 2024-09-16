use crate::{lang::typed::{expr::TExpr, root::Project, stmt::TStmt, types::Type}, scratch::{Assembly, Expr, Statement}};

/// Translates a project into scratch assembly
pub fn translate(project: Project) -> Assembly {
    let mut stmts = vec![Statement::ClearList { ident: "console".to_string() }]; // first statement is to clear the console
    
    // translate the main procedure's statements
    for stmt in project.main.0.stmts {
        tstmt(stmt.0.0, &mut stmts);
    }
    if let Some(stmt) = project.main.0.tail {
        tstmt(stmt.0.0, &mut stmts);
    }

    Assembly {
        stmts,
        variables: Vec::new(),
        lists: vec!["console".to_string()],
        procedures: Vec::new(),
    }
}

const NIL: &str = "<nil>";

/// Translates a statement
pub fn tstmt(stmt: TStmt, stmts: &mut Vec<Statement>) {
    match stmt {
        TStmt::Expr(expr) => {texpr(expr, stmts);},
        TStmt::VarDeclare { ident, value } => {
            // if it's a list then declare the list by making an empty list then setting all the values in it
            if let Type::List(_) = value.1 {
                // pull the expressions from the list
                let exprs = match value.0.0 {
                    TExpr::List(_, exprs) => exprs,
                    TExpr::VarGet { .. } => panic!("todo: need to implement loops first"),
                    _ => unreachable!(),
                };

                // iterate through the exprs and add them to the list
                for (i, expr) in exprs.into_iter().enumerate() {
                    let expr = texpr(expr.0, stmts);
                    let stmt = Statement::InsertList { ident: ident.clone(), value: expr, idx:Expr::PosInteger(i as u32 + 1)  };
                    stmts.push(stmt);
                }
            } else {
                let stmt = Statement::SetVar { ident, value: texpr(value.0.0, stmts) };
                stmts.push(stmt);
            }
            
        },
        TStmt::VarMutate { ident, value } => {
            // if it's a list then clear it and set all the values within it
            if let Type::List(_) = value.1 {
                // clear the list first
                stmts.push(Statement::ClearList { ident: ident.clone() });
                
                // pull the expressions from the list
                let exprs = match value.0.0 {
                    TExpr::List(_, exprs) => exprs,
                    TExpr::VarGet { .. } => panic!("todo: need to implement loops first"),
                    _ => unreachable!()
                };

                // iterate through the exprs and add them to the list
                for (i, expr) in exprs.into_iter().enumerate() {
                    let expr = texpr(expr.0, stmts);
                    let stmt = Statement::InsertList { ident: ident.clone(), value: expr, idx:Expr::PosInteger(i as u32 + 1)  };
                    stmts.push(stmt);
                }
            } else {
                let stmt = Statement::SetVar { ident, value: texpr(value.0.0, stmts) };
                stmts.push(stmt);
            }
        },
    };
}

/// Translates an expr
pub fn texpr(expr: TExpr, stmts: &mut Vec<Statement>) -> Expr {
    
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
                tstmt(stmt, stmts);
            }

            // return tail statement
            match block.tail {
                Some(((TStmt::Expr(tail), _), _)) => texpr(tail, stmts),
                _ => Expr::String("<nil>".to_string()),
            }
        },

        // builtin-function calls
        E::BuiltinFnCall(call) => {
            use crate::lang::typed::builtin::TBuiltinFnCall as B;
            match *call {
                // convert the `as_str` builtin to it's scratch counterpart
                B::AsString((expr, _)) => texpr(expr, stmts),

                // convert the `input` builtin to it's scratch counterpart
                B::Input((expr, _)) => {
                    let prompt = texpr(expr, stmts);
                    stmts.push(Statement::Ask { prompt });
                    Expr::Answer
                },

                // convert the `timer` builtin to it's scratch counterpart
                B::Timer => Expr::Timer,
                
                // convert the `println` builtin to it's scratch counterpart
                B::PrintLn(args) => {
                    let stmt = match args {
                        Some((expr, _)) => Statement::PushList { ident: "console".to_string(), value: texpr(expr, stmts) },
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

            let lhs = texpr(lhs, stmts);
            let rhs = texpr(rhs, stmts);

            Expr::Concat(Box::new(lhs), Box::new(rhs))
        },

        // maths single-space
        E::Pos(expr) => texpr(expr.0.0, stmts),
        E::Neg(expr) => Expr::Mul(
            Box::new(texpr(expr.0.0, stmts)),
            Box::new(Expr::Integer(-1)),
        ),

        // maths double-space
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

        // Getting Variables
        E::VarGet { ident, var_type } => {
            // if list, use placeholder
            if let Type::List(_) = var_type {
                Expr::String("<list>".to_string())
            } else {
                Expr::Variable { ident }
            }
        },

        // Lists
        E::List(_, _) => Expr::String("<list>".to_string()),

        _ => todo!(),
    }
}
