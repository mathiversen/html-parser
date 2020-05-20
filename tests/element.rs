use html_parser::{HtmlParser, Result};
use indoc::indoc;
use insta::assert_debug_snapshot;

#[test]
fn it_can_parse_one_element() -> Result<()> {
    let markup = "<html></html>";
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_one_element_upper_case() -> Result<()> {
    let markup = "<HTML></HTML>";
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case() -> Result<()> {
    let markup = "<Html></Html>";
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case_numbers() -> Result<()> {
    let markup = "<Header1></Header1>";
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case_numbers_symbols() -> Result<()> {
    let markup = "<Head_Er-1></Head_Er-1>";
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_multiple_elements() -> Result<()> {
    let markup = "<div/><div/>";
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_multiple_open_elements() -> Result<()> {
    let markup = "<div></div><div></div>";
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_nested_elements() -> Result<()> {
    let markup = indoc!(
        r"
        <div>
            <div />
        </div>
    "
    );
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_nested_elements_mixed_children() -> Result<()> {
    let markup = indoc!(
        r"
        <div>
            <!--comment-->
            <div/>
            Hello
            <div>
                World
            </div>
        </div>
    "
    );
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
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
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
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
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
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
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_skips_dangling_elements() -> Result<()> {
    let markup = indoc!(
        "
        <div id='123'></div>
        </div>
        <div id='321'></div>
    "
    );
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_errors_when_multiple_elements_dont_match() {
    assert!(HtmlParser::parse("<div></span><div></div>").is_err());
}
#[test]
fn it_errors_when_multiple_nested_elements_dont_match() {
    assert!(HtmlParser::parse("<div><div><div><div></div></div_error></div></div>").is_err());
}
