use crate::{lang::typed::{expr::TExpr, root::Project, stmt::TStmt, types::Type}, scratch::{Assembly, Condition, Expr, Procedure, Statement}};

/// Translates a project into scratch assembly
pub fn translate(project: Project) -> Assembly {
    let mut stmts = vec![Statement::ClearList { ident: "console".to_string() }]; // first statement is to clear the console
    let mut tmp_binds = 0; // temporary binding idx

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
    for stmt in project.main.stmts {
        tstmt(stmt.0.0, &mut stmts, &mut tmp_binds);
    }
    if let Some(stmt) = project.main.tail {
        tstmt(stmt.0.0, &mut stmts, &mut tmp_binds);
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
pub fn tstmt(stmt: TStmt, stmts: &mut Vec<Statement>, tmp_binds: &mut usize) {
    match stmt {
        TStmt::Expr(expr) => {texpr(expr, stmts, tmp_binds);},
        TStmt::VarDeclare { ident, value } => {
            // if it's a list then declare the list by making an empty list then setting all the values in it
            if let Type::List(_) = value.1 {
                // translate the list to a var-get
                let list = tlist(value.0.0, stmts, tmp_binds);

                // generate a temporary binding for the loop index
                *tmp_binds += 1;
                let loop_idx = get_tmp_binds_id(*tmp_binds);
                stmts.push(Statement::SetVar { ident: loop_idx.clone(), value: Expr::PosInteger(1) }); // lists start at 1
                
                // loop through the list and append the elements
                stmts.push(Statement::RepeatUntil {
                    // make sure it's within the bounds of the list
                    condition: Condition::GreaterThan(Expr::Variable { ident: loop_idx.clone() }, Expr::ListLength { ident: list.clone() }),
                    body: vec![
                        // push the list element to the new list
                        Statement::PushList {
                            ident,
                            value: Expr::ListElement { ident: list, idx: Box::new(Expr::Variable { ident: loop_idx.clone() }) },
                        },
                        // update the index
                        Statement::SetVar { ident: loop_idx.clone(), value: Expr::Add(Box::new(Expr::Variable { ident: loop_idx }), Box::new(Expr::PosInteger(1))) },
                    ],
                })
            } else {
                let stmt = Statement::SetVar { ident, value: texpr(value.0.0, stmts, tmp_binds) };
                stmts.push(stmt);
            }
            
        },
        TStmt::VarMutate { ident, value } => {
            // if it's a list then clear it and set all the values within it
            if let Type::List(_) = value.1 {
                // clear the list first
                stmts.push(Statement::ClearList { ident: ident.clone() });

                // translate the list to a var-get
                let list = tlist(value.0.0, stmts, tmp_binds);

                // generate a temporary binding for the loop index
                *tmp_binds += 1;
                let loop_idx = get_tmp_binds_id(*tmp_binds);
                stmts.push(Statement::SetVar { ident: loop_idx.clone(), value: Expr::PosInteger(1) });
                
                // loop through the list and append the elements
                stmts.push(Statement::RepeatUntil {
                    // make sure it's within the bounds of the list
                    condition: Condition::GreaterThan(Expr::Variable { ident: loop_idx.clone() }, Expr::ListLength { ident: list.clone() }),
                    body: vec![
                        // push the list element to the new list
                        Statement::PushList {
                            ident,
                            value: Expr::ListElement { ident: list, idx: Box::new(Expr::Variable { ident: loop_idx.clone() }) },
                        },
                        // update the index
                        Statement::SetVar { ident: loop_idx.clone(), value: Expr::Add(Box::new(Expr::Variable { ident: loop_idx }), Box::new(Expr::PosInteger(1))) },
                    ],
                })
            } else {
                let stmt = Statement::SetVar { ident, value: texpr(value.0.0, stmts, tmp_binds) };
                stmts.push(stmt);
            }
        },
        TStmt::If { cond, body, otherwise } => {
            // translate the condition
            let cond = tcond(cond.0.0, stmts, tmp_binds);

            // collect the body statements
            let mut body_stmts = Vec::new();
            tstmt(body.0, &mut body_stmts, tmp_binds);

            // collect the else statements if there are any
            let otherwise_stmts = if let Some(otherwise) = otherwise {
                let mut otherwise_stmts = Vec::new();
                tstmt(otherwise.0, &mut otherwise_stmts, tmp_binds);

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
            let cond = Condition::Not(Box::new(tcond(cond.0.0, stmts, tmp_binds)));

            // collect the body statements
            let mut body_stmts = Vec::new();
            tstmt(body.0, &mut body_stmts, tmp_binds);

            // return completed while statement
            let stmt = Statement::RepeatUntil { condition: cond, body: body_stmts };
            stmts.push(stmt);
        },
    };
}

/// Get a unique var name from a temporary bindings index
#[inline]
fn get_tmp_binds_id(tmp_binds: usize) -> String { // might cause performance issues where there are too many variables and lists
    format!("%{tmp_binds}")
}

/// Translates a list (creates a temporary bind) and returns the name of that binding
pub fn tlist(list: TExpr, stmts: &mut Vec<Statement>, tmp_binds: &mut usize) -> String {
    match list {
        // if it's a variable just return the variable identifier
        TExpr::VarGet { ident, .. } => return ident,
        // literal list
        TExpr::List(_, exprs) => {
            // generate a new temporary binding index
            *tmp_binds += 1;
            let ident = get_tmp_binds_id(*tmp_binds);

            // iterate through the exprs and add them to the list
            for (i, expr) in exprs.into_iter().enumerate() {
                let expr = texpr(expr.0, stmts, tmp_binds);
                let stmt = Statement::InsertList { ident: ident.clone(), value: expr, idx:Expr::PosInteger(i as u32 + 1)  };
                stmts.push(stmt);
            }

            ident
        },

        // no support for anything else yet
        _ => todo!(),
    }
}

/// Translates a condition
pub fn tcond(cond: TExpr, stmts: &mut Vec<Statement>, tmp_binds: &mut usize) -> Condition {
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
            let lhs = texpr(lhs.0.0, stmts, tmp_binds);
            let rhs = texpr(rhs.0.0, stmts, tmp_binds);
            Condition::EqualTo(lhs, rhs)
        },
        TExpr::NE(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts, tmp_binds);
            let rhs = texpr(rhs.0.0, stmts, tmp_binds);
            Condition::Not(Box::new(Condition::EqualTo(lhs, rhs)))
        },

        TExpr::GT(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts, tmp_binds);
            let rhs = texpr(rhs.0.0, stmts, tmp_binds);
            Condition::GreaterThan(lhs, rhs)
        },
        TExpr::LT(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts, tmp_binds);
            let rhs = texpr(rhs.0.0, stmts, tmp_binds);
            Condition::LessThan(lhs, rhs)
        },

        TExpr::GTE(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts, tmp_binds);
            let rhs = texpr(rhs.0.0, stmts, tmp_binds);
            Condition::Or(
                Box::new(Condition::GreaterThan(lhs.clone(), rhs.clone())),
                Box::new(Condition::EqualTo(lhs, rhs)),
            )
        },
        TExpr::LTE(lhs, rhs) => {
            let lhs = texpr(lhs.0.0, stmts, tmp_binds);
            let rhs = texpr(rhs.0.0, stmts, tmp_binds);
            Condition::Or(
                Box::new(Condition::LessThan(lhs.clone(), rhs.clone())),
                Box::new(Condition::EqualTo(lhs, rhs)),
            )
        },

        TExpr::And(lhs, rhs) => {
            let lhs = tcond(lhs.0.0, stmts, tmp_binds);
            let rhs = tcond(rhs.0.0, stmts, tmp_binds);
            Condition::And(Box::new(lhs), Box::new(rhs))
        },
        TExpr::Or(lhs, rhs) => {
            let lhs = tcond(lhs.0.0, stmts, tmp_binds);
            let rhs = tcond(rhs.0.0, stmts, tmp_binds);
            Condition::Or(Box::new(lhs), Box::new(rhs))
        },

        TExpr::Not(cond) => {
            let cond = tcond(cond.0.0, stmts, tmp_binds);
            Condition::Not(Box::new(cond))
        },

        _ => panic!("should be covered by the type system (must be valid)"),
    }
}

/// Translates an expr
pub fn texpr(expr: TExpr, stmts: &mut Vec<Statement>, tmp_binds: &mut usize) -> Expr {
    
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
                tstmt(stmt, stmts, tmp_binds);
            }

            // return tail statement
            match block.tail {
                Some(((TStmt::Expr(tail), _), _)) => texpr(tail, stmts, tmp_binds),
                _ => Expr::String("<nil>".to_string()),
            }
        },

        // builtin-function calls
        E::BuiltinFnCall(call) => {
            use crate::lang::typed::builtin::TBuiltinFnCall as B;
            match *call {
                // convert the `as_str` builtin to it's scratch counterpart
                B::AsString((expr, _)) => texpr(expr, stmts, tmp_binds),

                // convert the `input` builtin to it's scratch counterpart
                B::Input((expr, _)) => {
                    let prompt = texpr(expr, stmts, tmp_binds);
                    stmts.push(Statement::Ask { prompt });
                    Expr::Answer
                },

                // convert the `timer` builtin to it's scratch counterpart
                B::Timer => Expr::Timer,
                
                // convert the `println` builtin to it's scratch counterpart
                B::PrintLn(args) => {
                    let stmt = match args {
                        Some((expr, _)) => Statement::PushList { ident: "console".to_string(), value: texpr(expr, stmts, tmp_binds) },
                        None => Statement::PushList { ident: "console".to_string(), value: Expr::String(String::new()) },
                    };

                    stmts.push(stmt);
                    Expr::String(NIL.to_string())
                },

                // convert the `panic` builtin to it's scratch ounterpart
                B::Panic(span, arg) => {
                    let arg = match arg {
                        Some(arg) => texpr(arg.0, stmts, tmp_binds),
                        None => Expr::String("explicit panic".to_string()),
                    };

                    // set the panic message
                    stmts.push(Statement::SetVar { ident: PANIC_NAME.to_string(), value: {
                        Expr::Concat(
                            Box::new(Expr::String(format!("explicit panic at <{span:?}>: "))),
                            Box::new(arg),
                        )
                    } });

                    // call the panic
                    stmts.push(Statement::CallProcedure { ident: "$panic".to_string() });

                    Expr::String(NIL.to_string())
                },

                // list builtin-funcs

                // convert the `list_len` builtin to it's scratch counterpart
                B::ListLen(expr) => {
                    // translate list to get-var
                    let list = tlist(expr.0, stmts, tmp_binds);
                    // return operation on that list
                    Expr::ListLength { ident: list }
                },

                // convert the `list_len` builtin to it's scratch counterpart
                B::ListPush { list, expr } => {
                    // translate the expr & list
                    let list = tlist(list.0, stmts, tmp_binds);
                    let expr = texpr(expr.0, stmts, tmp_binds);

                    // push to the list
                    stmts.push(Statement::PushList {
                        ident: list,
                        value: expr,
                    });

                    // return nill
                    Expr::String(NIL.to_string())
                },

                // convert the `list_get` builtin to it's scratch counterpart
                B::ListGet { span, list, idx } => {
                    // translate list to get-var
                    let list = tlist(list.0, stmts, tmp_binds);
                    
                    // translate the list idx (+1 due to their lists indexs starting at 1 instead of 0)
                    let idx = texpr(idx.0, stmts, tmp_binds);
                    let idx_plus = Expr::Add(Box::new(idx.clone()), Box::new(Expr::PosInteger(1)));

                    // add bounds checking statement (negative)
                    stmts.push(Statement::If {
                        condition: Condition::LessThan(
                            idx.clone(),
                            Expr::PosInteger(0),
                        ),
                        body: vec![
                            Statement::SetVar {
                                ident: PANIC_NAME.to_string(),
                                value: Expr::Concat(
                                    Box::new(Expr::String(format!("panic at <{span:?}>: index cannot be negative: idx: "))),
                                    Box::new(idx.clone()),
                                ),
                            },
                            Statement::CallProcedure { ident: "$panic".to_string() }
                        ],
                    });

                    // add bounds checking statement (larger than length)
                    stmts.push(Statement::If {
                        condition: Condition::GreaterThan(
                            idx_plus.clone(),
                            Expr::ListLength { ident: list.clone() },
                        ),
                        body: vec![
                            Statement::SetVar { // the pain of formatting a string in scratch
                                ident: PANIC_NAME.to_string(),
                                value: Expr::Concat(
                                    Box::new(Expr::Concat(
                                        Box::new(Expr::Concat(
                                            Box::new(Expr::String(format!("panic at <{span:?}>: index out of bounds: len is "))),
                                            Box::new(Expr::ListLength { ident: list.clone() }),
                                        )),
                                        Box::new(Expr::String(" but the index is ".to_string())),
                                    )),
                                    Box::new(idx),
                                ),
                            },
                            Statement::CallProcedure { ident: "$panic".to_string() }
                        ],
                    });

                    // return operation on that list
                    Expr::ListElement { ident: list, idx: Box::new(idx_plus) }
                },

                // convert the `list_len` builtin to it's scratch counterpart
                B::ListInsert { span, list, idx, expr } => {
                    // translate the idx, expr & list
                    let list = tlist(list.0, stmts, tmp_binds);
                    let expr = texpr(expr.0, stmts, tmp_binds);
                    
                    // translate the list idx (+1 due to their lists indexs starting at 1 instead of 0)
                    let idx = texpr(idx.0, stmts, tmp_binds);
                    let idx_plus = Expr::Add(Box::new(idx.clone()), Box::new(Expr::PosInteger(1)));

                    // add bounds checking statement (negative)
                    stmts.push(Statement::If {
                        condition: Condition::LessThan(
                            idx.clone(),
                            Expr::PosInteger(0),
                        ),
                        body: vec![
                            Statement::SetVar {
                                ident: PANIC_NAME.to_string(),
                                value: Expr::Concat(
                                    Box::new(Expr::String(format!("panic at <{span:?}>: index cannot be negative: idx: "))),
                                    Box::new(idx.clone()),
                                ),
                            },
                            Statement::CallProcedure { ident: "$panic".to_string() }
                        ],
                    });

                    // add bounds checking statement (larger than length)
                    stmts.push(Statement::If {
                        condition: Condition::GreaterThan(
                            idx_plus.clone(),
                            Expr::ListLength { ident: list.clone() },
                        ),
                        body: vec![
                            Statement::SetVar { // the pain of formatting a string in scratch
                                ident: PANIC_NAME.to_string(),
                                value: Expr::Concat(
                                    Box::new(Expr::Concat(
                                        Box::new(Expr::Concat(
                                            Box::new(Expr::String(format!("panic at <{span:?}>: index out of bounds: len is "))),
                                            Box::new(Expr::ListLength { ident: list.clone() }),
                                        )),
                                        Box::new(Expr::String(" but the index is ".to_string())),
                                    )),
                                    Box::new(idx),
                                ),
                            },
                            Statement::CallProcedure { ident: "$panic".to_string() }
                        ],
                    });

                    // insert the expr at that index in the list
                    stmts.push(Statement::InsertList {
                        ident: list,
                        value: expr,
                        idx: idx_plus,
                    });

                    // return nill
                    Expr::String(NIL.to_string())
                },
            }
        },

        // concat
        E::Concat(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, stmts, tmp_binds);
            let rhs = texpr(rhs, stmts, tmp_binds);

            Expr::Concat(Box::new(lhs), Box::new(rhs))
        },

        // maths single-space
        E::Pos(expr) => texpr(expr.0.0, stmts, tmp_binds),
        E::Neg(expr) => Expr::Mul(
            Box::new(texpr(expr.0.0, stmts, tmp_binds)),
            Box::new(Expr::Integer(-1)),
        ),

        // maths double-space
        E::Add(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, stmts, tmp_binds);
            let rhs = texpr(rhs, stmts, tmp_binds);

            Expr::Add(Box::new(lhs), Box::new(rhs))
        },
        E::Sub(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, stmts, tmp_binds);
            let rhs = texpr(rhs, stmts, tmp_binds);

            Expr::Sub(Box::new(lhs), Box::new(rhs))
        },
        E::Mul(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, stmts, tmp_binds);
            let rhs = texpr(rhs, stmts, tmp_binds);

            Expr::Mul(Box::new(lhs), Box::new(rhs))
        },
        E::Div(lhs, rhs) => {
            let (((lhs, _), _), ((rhs, _), _)) = (*lhs, *rhs);

            let lhs = texpr(lhs, stmts, tmp_binds);
            let rhs = texpr(rhs, stmts, tmp_binds);

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
        E::Bool(bool) => Expr::Condition(Box::new(tcond(TExpr::Bool(bool), stmts, tmp_binds))),
        E::EE(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::EE(lhs, rhs), stmts, tmp_binds))),
        E::NE(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::NE(lhs, rhs), stmts, tmp_binds))),
        E::GT(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::GT(lhs, rhs), stmts, tmp_binds))),
        E::LT(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::LT(lhs, rhs), stmts, tmp_binds))),
        E::GTE(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::GTE(lhs, rhs), stmts, tmp_binds))),
        E::LTE(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::LTE(lhs, rhs), stmts, tmp_binds))),
        E::And(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::And(lhs, rhs), stmts, tmp_binds))),
        E::Or(lhs, rhs) => Expr::Condition(Box::new(tcond(TExpr::Or(lhs, rhs), stmts, tmp_binds))),
        E::Not(cond) => Expr::Condition(Box::new(tcond(TExpr::Not(cond), stmts, tmp_binds))),

        _ => todo!(),
    }
}
