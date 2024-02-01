use roped::*;

mod base;

use base::*;

fn main() {
    loop {
        parse::run_console::<Container, EmptyState>(&mut EmptyState, "".into(), &[' '], &[';', '\n', 13u8 as char])
    }
}
