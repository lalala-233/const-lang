use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    let mut parser = const_lang::Parser::default();
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();

    let stdin = std::io::stdin();
    loop {
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut input)?;
        match parser.parse(&input) {
            Ok(s) if !s.is_empty() => writeln!(stdout, "{s}")?,
            Ok(_) => (),
            Err(e) => writeln!(stderr, "Error: {e}")?,
        }
        input.clear();
    }
}
