use roped::*;

mod base;

use base::*;

fn main() {
    loop {
        parse::run_console::<Container, EmptyState>(&mut EmptyState, "> ".into(), &[' '], &[';'])
    }
}
