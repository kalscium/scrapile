use std::{fs::File, io::Write, path::Path};
use json::{array, object, JsonValue};
use zip::{write::SimpleFileOptions, ZipWriter};

/// An expression in scratch (returns a value)
#[derive(Debug, Clone)]
pub enum Expr {
    // Atoms
    Number(f32),
    PosNumber(f32),
    PosInteger(u32),
    Integer(i32),
    String(String),

    // operations
    Condition(Box<Condition>), // conditions can be converted to strings as an expr Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),

    // list & variable operations
    Variable { ident: String },
    ListElement {
        ident: String,
        idx: Box<Expr>,
    },
    ListLength { ident: String },
}

/// A condition in scratch (different from an expression as it can only be used in if statements)
#[derive(Debug, Clone)]
pub enum Condition {
    // expr to expr conditinos
    MoreThan(Expr, Expr),
    LessThan(Expr, Expr),
    EqualTo(Expr, Expr),

    // condition to condition conditions
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

/// A statement in scratch (doesn't return anything)
#[derive(Debug, Clone)]
pub enum Statement {
    // If {
    //     condition: Condition,
    //     body: Vec<Statement>,
    //     /// else
    //     otherwise: Option<Vec<Statement>>,
    // },
    
    SetVar {
        ident: String,
        value: Expr,
    },
    GetVar { ident: String },
    ShowVar { ident: String },
    HideVar { ident: String },

    PushList {
        ident: String,
        value: Expr,
    },
    RemoveList {
        ident: String,
        idx: Expr,
    },
    ClearList(String),
    InsertList {
        ident: String,
        value: Expr,
        idx: Expr,
    },
    ReplaceList {
        ident: String,
        value: Expr,
        idx: Expr,
    },
    ShowList { ident: String },
    HideList { ident: String },
}

#[inline]
fn block_idx_to_id(idx: usize) -> String {
    format!("block_idx: {idx}")
}

/// The scratch equivelent of an 'assembler'; it takes in instructions that are very close to the scratch equivelant and generates the final `.sb3` project
///
/// it also requires a list of all the variable and list ids used in the statements
pub fn assemble(stmts: &[Statement], variables: &[String], lists: &[String]) -> JsonValue {
    // parse statements
    let mut blocks = Vec::new();
    let mut stmt_blocks = stmts.iter()
        .map(|stmt| parse_stmt(stmt, &mut blocks))
        .collect::<Vec<_>>();
    blocks.append(&mut stmt_blocks); // append the generated stmt blocks onto the blocks vector

    // generate the full json template
    let mut json = object! {
        targets: [
            {
                isStage: true,
                name: "Stage",
                variables: {}, // replaced later
                lists: {}, // replaced later
                broadcasts: {},
                blocks: {}, // replaced later
                comments: {},
                currentCostume: 0,
                costumes: [
                    {
                        name: "backdrop1",
                        dataFormat: "svg",
                        assetId: "cd21514d0531fdffb22204e0ec5ed84a",
                        md5ext: "cd21514d0531fdffb22204e0ec5ed84a.svg",
                        rotationCenterX: 240,
                        rotationCenterY: 180,
                    }
                ],
                sounds: [],
                volume: 100,
                layerOrder: 0,
                tempo: 60,
                videoTransparency: 50,
                videoState: "on",
                textToSpeechLanguage: null,
            },
        ],
        monitors: [], // replaced later
        extensions: [],
        meta: {
            semver: "3.0.0",
            vm: "2.3.4",
        },
    };

    // insert variable and list definitions
    for var in variables {
        json["targets"][0]["variables"][var] = array! [
            **var,
            ""
        ];
    }
    for list in lists {
        json["targets"][0]["lists"][list] = array! [
            **list,
            []
        ];
    }

    // insert start block
    json["targets"][0]["blocks"]["startflag"] = object! {
        opcode: "event_whenflagclicked",
        next: block_idx_to_id(0),
        parent: null,
        inputs: {},
        fields: {},
        shadow: false,
        topLevel: true,
        x: 0,
        y: 0,
    };

    // insert rest of the blocks
    for (i, block) in blocks.into_iter().enumerate() {
        json["targets"][0]["blocks"][block_idx_to_id(i)] = block;
    }
        
    // return completed scratch json
    json
}

/// Takes the json output of `assemble` and writes it to a zip file of the path specified
pub fn write_to_zip(path: impl AsRef<Path>, json: JsonValue) -> Result<(), std::io::Error> {
    let mut zip = ZipWriter::new(File::create(path)?);

    // write the json
    zip.start_file("project.json", SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated))?;
    zip.write_all(json.to_string().as_bytes())?;

    // write the required svg asset
    zip.start_file("cd21514d0531fdffb22204e0ec5ed84a.svg", SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated))?;
    zip.write_all(include_str!("asset.svg").as_bytes())?;

    // finish
    zip.finish()?;
    Ok(())
}

/// Parses a scratch statement and outupts the generated json
fn parse_stmt(stmt: &Statement, blocks: &mut Vec<JsonValue>) -> JsonValue {
    use Statement as S;

    match stmt {
        S::PushList { ident, value } => {
            object! {
                opcode: "data_addtolist",
                next: block_idx_to_id(blocks.len()+1), // set the next block to a future block in the block vector
                parent: null,
                inputs: {
                    ITEM: [
                        1,
                        parse_expr(value.clone(), blocks),
                    ],
                },
                fields: {
                    LIST: [
                        **ident,
                        ""
                    ],
                },
                shadow: false,
                topLevel: false,
            }
        },
        _ => todo!(),
    }
}

/// Parses a scratch expression and outputs the generated json
/// (requires a mutable reference to the block vector to add addtional blocks for multi-step exressions)
fn parse_expr(expr: Expr, blocks: &mut Vec<JsonValue>) -> JsonValue {
    use Expr as E;

    match expr {
        // basic numbers & strings
        E::Number(num) => array![ 4, num ],
        E::PosNumber(num) => array![ 5, num ],
        E::PosInteger(num) => array![ 6, num ],
        E::Integer(num) => array![ 7, num ],
        E::String(num) => array![ 10, num ],

        // variables and lists
        E::Variable { ident } => array![ 12, ident ],
        E::ListElement { ident, idx } => {
            let json = object! {
                opcode: "data_itemoflist",
                next: null,
                parent: null,
                inputs: {
                    INDEX: [
                        1,
                        parse_expr((*idx).clone(), blocks),
                    ]
                },
                fields: {
                    LIST: [
                        ident,
                        "",
                    ]
                },
                shadow: false,
                topLevel: false,
            };
            blocks.push(json);

            block_idx_to_id(blocks.len()-1).into()
        },
        E::ListLength { ident } => {
            let json = object! {
                opcode: "data_lengthoflist",
                next: null,
                parent: null,
                inputs: {},
                fields: {
                    LIST: [
                        ident,
                        "",
                    ],
                },
            };
            blocks.push(json);

            block_idx_to_id(blocks.len()-1).into()
        }

        _ => todo!(),
    }
}
