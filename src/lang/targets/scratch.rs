use crate::{lang::typed::{expr::TExpr, root::Project, stmt::TStmt, types::Type}, scratch::{Assembly, Condition, Expr, Procedure, Statement}};

/// Translates a project into scratch assembly
pub fn translate(project: Project) -> Assembly {
    let mut stmts = vec![Statement::ClearList { ident: "console".to_string() }]; // first statement is to clear the console

    let procedures = vec![
        // panic
        Procedure {
            ident: "$panic".to_string(),
            body: vec![
                Statement::PushList { ident: "console".to_string(), value: Expr::Variable { ident: PANIC_NAME.to_string() } },
                Statement::StopAll,
            ],
        },
    ];
    
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
        lists: Vec::new(),
        procedures,
    }
}

const NIL: &str = "<nil>";
const PANIC_NAME: &str = "$panic$msg";

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
        TStmt::If { cond, body, otherwise } => {
            // translate the condition
            let cond = tcond(cond.0.0, stmts);

            // collect the body statements
            let mut body_stmts = Vec::new();
            tstmt(body.0, &mut body_stmts);

            // collect the else statements if there are any
            let otherwise_stmts = if let Some(otherwise) = otherwise {
                let mut otherwise_stmts = Vec::new();
                tstmt(otherwise.0, &mut otherwise_stmts);

                Some(otherwise_stmts)
            } else {
                None
            };

            // if there's an else statement then return an ifelse otherwise a normal if
            let stmt = if let Some(otherwise_stmts) = otherwise_stmts {
                Statement::IfElse { condition: cond, body: body_stmts, otherwise: otherwise_stmts }
            } else {
                Statement::If { condition: cond, body: body_stmts }
            };
            stmts.push(stmt);
        },
        TStmt::While { cond, body } => {
            // translate the condition (not as there is only repeatuntil)
            let cond = Condition::Not(Box::new(tcond(cond.0.0, stmts)));

            // collect the body statements
            let mut body_stmts = Vec::new();
            tstmt(body.0, &mut body_stmts);

            // return completed while statement
            let stmt = Statement::RepeatUntil { condition: cond, body: body_stmts };
            stmts.push(stmt);
        },
    };
}

/// Translates a condition
pub fn tcond(cond: TExpr, stmts: &mut Vec<Statement>) -> Condition {
    match cond {
        TExpr::Bool(bool) => Condition::EqualTo(
            if bool {
                Expr::String("true".to_string())
            } else {
                Expr::String("false".to_string())
            },
            Expr::String("true".to_string()),
        ),

        TExpr::VarGet { ident, .. } => Condition::EqualTo(Expr::Variable { ident }, Expr::String("true".to_string())),

        TExpr::EE(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts);
            let rhs = texpr(rhs.0.0, stmts);
            Condition::EqualTo(lhs, rhs)
        },
        TExpr::NE(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts);
            let rhs = texpr(rhs.0.0, stmts);
            Condition::Not(Box::new(Condition::EqualTo(lhs, rhs)))
        },

        TExpr::GT(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts);
            let rhs = texpr(rhs.0.0, stmts);
            Condition::GreaterThan(lhs, rhs)
        },
        TExpr::LT(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts);
            let rhs = texpr(rhs.0.0, stmts);
            Condition::LessThan(lhs, rhs)
        },

        TExpr::GTE(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts);
            let rhs = texpr(rhs.0.0, stmts);
            Condition::Or(
                Box::new(Condition::GreaterThan(lhs.clone(), rhs.clone())),
                Box::new(Condition::EqualTo(lhs, rhs)),
            )
        },
        TExpr::LTE(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts);
            let rhs = texpr(rhs.0.0, stmts);
            Condition::Or(
                Box::new(Condition::LessThan(lhs.clone(), rhs.clone())),
                Box::new(Condition::EqualTo(lhs, rhs)),
            )
        },

        TExpr::And(lhs, rhs) => {
            let lhs = tcond(lhs.0.0, stmts);
            let rhs = tcond(rhs.0.0, stmts);
            Condition::And(Box::new(lhs), Box::new(rhs))
        },
        TExpr::Or(lhs, rhs) => {
            let lhs = tcond(lhs.0.0, stmts);
            let rhs = tcond(rhs.0.0, stmts);
            Condition::Or(Box::new(lhs), Box::new(rhs))
        },

        TExpr::Not(cond) => {
            let cond = tcond(cond.0.0, stmts);
            Condition::Not(Box::new(cond))
        },

        _ => panic!("should be covered by the type system (must be valid)"),
    }
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

                // convert the `panic` builtin to it's scratch ounterpart
                B::Panic(span, arg) => {
                    let arg = match arg {
                        Some(arg) => texpr(arg.0, stmts),
                        None => Expr::String("explicit panic".to_string()),
                    };

                    // set the panic message
                    stmts.push(Statement::SetVar { ident: PANIC_NAME.to_string(), value: {
                        Expr::Concat(
                            Box::new(Expr::String(format!("panic at <{span:?}>: "))),
                            Box::new(arg),
                        )
                    } });

                    // call the panic
                    stmts.push(Statement::CallProcedure { ident: "$panic".to_string() });

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

        // Conditions
        E::Bool(bool) => Expr::Condition(Box::new(tcond(TExpr::Bool(bool), stmts))),
        E::EE(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::EE(lhs, rhs), stmts))),
        E::NE(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::NE(lhs, rhs), stmts))),
        E::GT(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::GT(lhs, rhs), stmts))),
        E::LT(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::LT(lhs, rhs), stmts))),
        E::GTE(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::GTE(lhs, rhs), stmts))),
        E::LTE(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::LTE(lhs, rhs), stmts))),
        E::And(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::And(lhs, rhs), stmts))),
        E::Or(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::Or(lhs, rhs), stmts))),
        E::Not(cond) => Expr::Condition(Box::new(tcond(TExpr::Not(cond), stmts))),

        _ => todo!(),
    }
}
