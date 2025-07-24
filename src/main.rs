use std::env;
use shell_escape::escape;
fn main() {
    let mut output: Vec<_> = Vec::with_capacity(env::args().len() - 1);
    for arg in env::args().into_iter().skip(1) {
        output.push(escape(arg.into()));
    }
    print!("{}", output.join(" "));
}
