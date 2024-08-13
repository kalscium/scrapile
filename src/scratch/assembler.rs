use json::{array, object, JsonValue::{self, Null}};
use crate::scratch::block_idx_to_id;
use super::{parse_stmt, Statement};

/// The scratch equivelent of an 'assembler'; it takes in instructions that are very close to the scratch equivelant and generates the final `.sb3` project
///
/// it also requires a list of all the variable and list ids used in the statements
pub fn assemble(stmts: &[Statement], variables: &[String], lists: &[String]) -> JsonValue {
    // parse statements
    let mut expr_blocks = Vec::new();
    let stmt_blocks = stmts.iter()
        .map(|stmt| parse_stmt(stmt, &mut expr_blocks))
        .collect::<Vec<_>>();

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

    // insert the statement blocks
    for (i, mut stmt_block) in stmt_blocks.into_iter().enumerate() {
        let idx = expr_blocks.len() + i; // true block index with expr blocks included
        
        // update the link to the next block
        stmt_block["next"] = block_idx_to_id(idx+1).into();
        // set other boilerplate fields
        stmt_block["shadow"] = false.into();
        stmt_block["topLevel"] = false.into();
        stmt_block["parent"] = Null;

        // write stmt block to the main json
        json["targets"][0]["blocks"][block_idx_to_id(idx)] = stmt_block;
    }
    // insert the expr blocks
    for (i, expr_block) in expr_blocks.into_iter().enumerate() {
        json["targets"][0]["blocks"][block_idx_to_id(i)] = expr_block;
    }

    // return completed scratch json
    json
}
