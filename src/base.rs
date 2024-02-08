mod char_convert;
mod radix_convert;

use roped::*;

#[allow(dead_code)]
#[derive(Bundle)]
#[bundle(state = "EmptyState")]
pub enum Container {
    #[bundle(prefix = ":")]
    Sys(Sys),
    #[bundle(name = "convert")]
    Convert(char_convert::Convert),
    #[bundle(name = "radix")]
    Radix(radix_convert::Convert),
}

#[allow(dead_code)]
#[derive(Bundle)]
#[bundle(state = "EmptyState")]
pub enum Sys {
    #[bundle(name = "quit")]
    Quit(Quit),
}

#[allow(dead_code)]
#[derive(Strand)]
#[strand(state = "EmptyState", action = "action")]
pub struct Quit;

impl Quit {
    fn action(&self, _: &mut EmptyState) -> Result<(), String> {
        println!("Exiting program...");
        std::process::exit(0)
    }
}
