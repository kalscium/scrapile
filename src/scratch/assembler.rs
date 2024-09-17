use json::{array, object, JsonValue::{self, Null}};
use super::{parse_procedure, parse_stmt, Procedure, Statement};

#[inline]
pub(super) fn expr_idx_to_id(idx: usize) -> String {
    format!("expr_idx: {idx}")
}

#[inline]
fn stmt_idx_to_id(idx: usize) -> String {
    format!("stmt_idx: {idx}")
}

/// Scratch assembly, the closest thing to scratch blocks without being actual scratch blocks
#[derive(Debug)]
pub struct Assembly {
    pub stmts: Vec<Statement>,
    pub variables: Vec<String>,
    pub lists: Vec<String>,
    pub procedures: Vec<Procedure>,
}

/// The scratch equivelent of an 'assembler'; it takes in instructions that are very close to the scratch equivelant and generates the final `.sb3` project
///
/// it also requires a list of all the variable and list ids used in the statements
pub fn assemble(assembly: Assembly) -> JsonValue {
    // parse statements
    let mut expr_blocks = Vec::new();
    let stmt_blocks = assembly.stmts.into_iter()
        .map(|stmt| parse_stmt(stmt, &mut expr_blocks))
        .collect::<Vec<_>>();
    // parse procedures
    for procedure in assembly.procedures.into_iter() {
        parse_procedure(procedure, &mut expr_blocks);
    }

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
                        name: "backdrop1", dataFormat: "svg",
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
        monitors: [],
        extensions: [],
        meta: {
            semver: "3.0.0",
            vm: "2.3.4",
        },
    };

    // insert variable and list definitions
    for var in assembly.variables.iter() {
        json["targets"][0]["variables"][var] = array! [
            **var,
            ""
        ];
    }
    for list in assembly.lists.iter() {
        json["targets"][0]["lists"][list] = array! [
            **list,
            []
        ];
    }

    // insert start block
    json["targets"][0]["blocks"]["startflag"] = object! {
        opcode: "event_whenflagclicked",
        next: stmt_idx_to_id(0),
        parent: null,
        inputs: {},
        fields: {},
        shadow: false,
        topLevel: true,
        x: 0,
        y: 0,
    };

    // insert the statement blocks
    for (idx, mut stmt_block) in stmt_blocks.into_iter().enumerate() {
        // update the link to the next block
        stmt_block["next"] = stmt_idx_to_id(idx+1).into();
        // set other boilerplate fields
        stmt_block["shadow"] = false.into();
        stmt_block["topLevel"] = false.into();
        stmt_block["parent"] = Null;

        // write stmt block to the main json
        json["targets"][0]["blocks"][stmt_idx_to_id(idx)] = stmt_block;
    }
    // insert the expr blocks
    for (i, expr_block) in expr_blocks.into_iter().enumerate() {
        json["targets"][0]["blocks"][expr_idx_to_id(i)] = expr_block;
    }

    // return completed scratch json
    json
}
