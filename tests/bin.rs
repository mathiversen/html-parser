use html_parser::Result;
use indoc::indoc;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn it_prints_out_processing_error() -> Result<()> {
    let html = indoc!(
        r#"
            <?xml version="1.0" ?>
            <div />
        "#
    );

    let mut file = NamedTempFile::new()?;
    file.write_all(html.as_bytes())?;

    let output = Command::new("./target/debug/examples/simple_parser")
        .arg("-d")
        .arg(file.path())
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.starts_with("# Failed to create element at rule: el_process_instruct"));
    Ok(())
}
