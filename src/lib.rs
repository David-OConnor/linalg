#[macro_use]
extern crate seed;
use seed::prelude::*;


// Model

struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
}

fn update(msg: Msg, model: Model) -> Update<Msg, Model> {
    match msg {
    }
}


// View

fn definition(description: &str, def: &str) -> El<Msg> {
    // todo: Could add $$ here.
    div![
        h5![description],
        span![format!("$${}$$", def)],
    ]
}

// \langle, \rangle, \lvert, and \rvert ^\dagger

fn view(_state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    div![
        h1!["Linear algebra cheatsheet"],
        p!["Intent: Provide a quick reference of definitions and identities that 
        are useful in formal, symbolic linear algebra"],
        section![

        ],

        section![

            definition(
                "Definition of matrix multiplication", 
                r"C_{ij} = \sum_k A_{ik} B_{kj}"
            ),

            definition(
                "Definition of unit matrix", 
                r"\mathbf{1A} = \mathbf{A1} = \mathbf{A}"
            ),
            
            definition(
                "Definition of inverse matrix", 
                r"\mathbf{A}^{-1} \mathbf{A} = \mathbf{A} \mathbf{A}^{-1} = 1"
            ),

            definition(
                "",
                // todo fix this one
                r"\langle a \lvert T \rvert b \rangle = \langle T^\dagger a \rvert b \rangle"
            ),

            definition(
                "",
                // todo fix this one
                r"\langle a \lvert T^\dagger \rvert c \rangle = \langle c \lvert T \rvert a \rangle^*"
            )

        ],


        footer![
            h3!["References"],
            ul![
                li!["Modern Quantum Chemistry, by Attila Szabo and Neil S. Ostlund"]
            ]
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();

}