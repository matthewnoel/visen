use visen::ScriptError;
use std::env;

fn main() -> Result<(), ScriptError> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 && &args[1] ==  "init" {
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
