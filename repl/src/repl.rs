use lexer::lexer;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
const PROMPT: &str = ">> ";

pub fn start<R: Read, W: Write>(input: R, mut output: W) -> std::io::Result<()> {
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    loop {
        output.write(PROMPT.as_bytes())?;
        output.flush()?;
        if let Some(Ok(line)) = lines.next() {
            let mut lex = lexer::Lexer::new(&line);
            loop {
                match lex.next_token() {
                    Ok(tok) => {
                        output.write(format!("{:?}\n", tok).as_bytes())?;
                    }
                    Err(lexer::LexErrorKind::Eof) => break,
                    Err(e) => {
                        eprintln!("ERROR: {:?}", e);
                        break;
                    }
                }
                output.flush()?;
            }
        }
    }
}
