use visen::ScriptError;

fn main() -> Result<(), ScriptError> {
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
