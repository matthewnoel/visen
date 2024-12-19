#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

use human_panic::{setup_panic, Metadata};
use std::env;
use visen::ScriptError;

fn main() -> Result<(), ScriptError> {
    setup_panic!(
        Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .authors("https://github.com/matthewnoel/visen/graphs/contributors")
            .homepage("https://github.com/matthewnoel/visen")
            .support("- Open an issue: https://github.com/matthewnoel/visen/issues")
    );

    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 && &args[1] == "init" {
        if args.len() > 3 {
            println!("Too many arguments. The extra arguments will be ignored.");
        }
        if args.len() == 2 {
            println!("Please provide a name for the project. E.g. visen init my-project");
            return Ok(());
        }
        let project_name = &args[2];
        visen::init(project_name)?;
    }

    visen::validate_command_is_running_inside_visen_project()?;

    let script = visen::build_script()?;
    match visen::write_html(&script) {
        Ok(_) => (),
        Err(_) => println!("Failed to write HTML"),
    };
    match visen::write_readme(&script) {
        Ok(_) => (),
        Err(_) => println!("Failed to write README"),
    }
    match clearscreen::clear() {
        Ok(_) => (),
        Err(_) => (),
    };
    println!("{}", script);
    return Ok(());
}
