use std::io::{self, LineWriter, Write};
use std::fs::{File, OpenOptions};

fn get_user_input() -> io::Result<Vec<String>> {
    let mut user_input = Vec::with_capacity(64);

    let stdin = io::stdin();
    let lines = stdin.lines();

    for line in lines {
        match line {
            Ok(line) =>
                match &line[..] {
                    "" => return Ok(user_input),
                    _ => user_input.push(line)
                }
            Err(e) => return Err(e),
        }
    }

    return Ok(user_input)
}

fn open_adoc() -> io::Result<LineWriter<File>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("x.adoc")?;

    Ok(LineWriter::new(file))
}

fn append_adoc_list_item(
    file: &mut LineWriter<File>,
    contents: &str) -> io::Result<()> {

    let contents = contents
        .replace("‘", "'")
        .replace("’", "'")
        .replace("“", "\"")
        .replace("”", "\"");

    file.write_all(b"\n")?;
    file.write_all(b"* ")?;
    file.write(contents.as_bytes())?;
    file.write_all(b"\n")?;
    

    Ok(())
}

fn main() -> io::Result<()> {
    
    let mut adoc = open_adoc()?;

    loop {
        let user_input = get_user_input()?;
        let joined = user_input.join(" ");
        let trimmed = joined.trim();

        if trimmed.is_empty() {
            continue;
        }

        append_adoc_list_item(
            &mut adoc,
            trimmed
        )?;
    }
}
