use html_parser::prelude::*;
use indoc::indoc;

#[test]
fn it_can_parse_empty_document() -> Result<()> {
    assert_eq!((), Ast::parse("", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_element() -> Result<()> {
    assert_eq!((), Ast::parse("<html></html>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_element_upper_case() -> Result<()> {
    assert_eq!((), Ast::parse("<HTML></HTML>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case() -> Result<()> {
    assert_eq!((), Ast::parse("<Html></Html>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case_numbers() -> Result<()> {
    assert_eq!((), Ast::parse("<Header1></Header1>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case_numbers_symbols() -> Result<()> {
    assert_eq!((), Ast::parse("<Head_er-1></Head_er-1>", false)?);
    Ok(())
}
#[test]
fn it_errors_when_case_dont_match() -> Result<()> {
    assert!(Ast::parse("<html></Html>", false).is_err());
    Ok(())
}
#[test]
fn it_errors_when_element_name_dont_match() -> Result<()> {
    assert!(Ast::parse("<html></div>", false).is_err());
    Ok(())
}
#[test]
fn it_can_parse_multiple_elements() -> Result<()> {
    assert_eq!((), Ast::parse("<div></div><div></div>", false)?);
    Ok(())
}
#[test]
fn it_errors_when_multiple_elements_dont_match() -> Result<()> {
    assert!(Ast::parse("<div></span><div></div>", false).is_err());
    Ok(())
}
#[test]
fn it_can_parse_one_comment() -> Result<()> {
    assert_eq!((), Ast::parse("<!-- hello !\"#/()= -->", false)?);
    Ok(())
}
#[test]
fn it_can_parse_multiple_comments() -> Result<()> {
    assert_eq!((), Ast::parse("<!--x--><!--y--><!--z-->", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_text() -> Result<()> {
    assert_eq!((), Ast::parse("hello world", false)?);
    Ok(())
}
#[test]
fn it_can_parse_multiple_rows_of_text() -> Result<()> {
    assert_eq!((), Ast::parse("hello\nworld\n!", false)?);
    Ok(())
}
#[test]
fn it_can_parse_element_comment_text() -> Result<()> {
    assert_eq!((), Ast::parse("<div></div><!--x-->hello", false)?);
    Ok(())
}
#[test]
fn it_can_parse_nested_elements() -> Result<()> {
    assert_eq!((), Ast::parse("<div><div></div></div>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_nested_elements_comments_text() -> Result<()> {
    assert_eq!(
        (),
        Ast::parse("<p id='body'><i>hello</i><!--x-->world</p>", false)?
    );
    Ok(())
}
#[test]
fn it_can_parse_nested_and_indented() -> Result<()> {
    let markup = indoc!(
        r#"
            <p id='body'>
                <i>hello</i>
                <!--x-->
                world
            </p>
        "#
    );
    assert_eq!((), Ast::parse(markup, false)?);
    Ok(())
}
#[test]
fn it_can_parse_deeply_nested() -> Result<()> {
    let markup = indoc!(
        r#"
            <div class='1'>
                <div class='1'>
                    <div class='1'>
                        <div class='1'>
                            <div class='1'>
                                <div class='1'>
                                    <div class='1'>
                                        <div class='1'>
                                            <!--this is deep-->
                                            hello world
                                        </div>
                                    </div>
                                </div>
                            </div> 
                        </div>
                    </div>
                </div>
            </div>
        "#
    );
    assert_eq!((), Ast::parse(markup, false)?);
    Ok(())
}
#[test]
fn it_can_parse_script_with_content() -> Result<()> {
    let markup = indoc!(
        r#"
            <script>
                const person_creator = ({ name, symtoms }) => {
                    let person = {}
                    person.name = name
                    person.symtoms = {}
                    for (symtom of symtoms) {
                        person.symtoms[symtom] = true
                    }
                    return person
                }
                
                const main = () => {
                    let name = 'mathias'
                    let symtoms = ['Dunning-Kruger', 'ACDC', 'Slacker']
                
                    setTimeout(() => {
                        let person = person_creator({ name, symtoms })
                        if (person.symtoms.hasOwnProperty('Dunning-Kruger')) {
                            console.log('yeah buddy, that\'s right')
                        }
                    }, 1337)
                }
                
                main()
            </script>
        "#
    );
    assert_eq!((), Ast::parse(markup, false)?);
    Ok(())
}
#[test]
fn it_can_parse_style_with_content() -> Result<()> {
    let markup = indoc!(
        r#"
            <style>
                :root {
                    --background-color: black;
                    --text-color: white;
                }
                body {
                    background: var(--background-color);
                    color: var(--text-color);
                }
            </style>
        "#
    );
    assert_eq!((), Ast::parse(markup, false)?);
    Ok(())
}
