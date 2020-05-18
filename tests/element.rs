use html_parser::prelude::*;
use indoc::indoc;

#[test]
fn it_can_parse_empty_document() {
    assert!(Ast::parse("", false).is_ok());
}
#[test]
fn it_can_parse_one_element() {
    assert!(Ast::parse("<html></html>", false).is_ok());
}
#[test]
fn it_can_parse_one_element_upper_case() {
    assert!(Ast::parse("<HTML></HTML>", false).is_ok());
}
#[test]
fn it_can_parse_one_element_mixed_case() {
    assert!(Ast::parse("<Html></Html>", false).is_ok());
}
#[test]
fn it_can_parse_one_element_mixed_case_numbers() {
    assert!(Ast::parse("<Header1></Header1>", false).is_ok());
}
#[test]
fn it_can_parse_one_element_mixed_case_numbers_symbols() {
    assert!(Ast::parse("<Head_er-1></Head_er-1>", false).is_ok());
}
#[test]
fn it_errors_when_case_dont_match() {
    assert!(Ast::parse("<html></Html>", false).is_err());
}
#[test]
fn it_errors_when_element_name_dont_match() {
    assert!(Ast::parse("<html></div>", false).is_err());
}
#[test]
fn it_can_parse_multiple_elements() {
    assert!(Ast::parse("<div></div><div></div>", false).is_ok());
}
#[test]
fn it_errors_when_multiple_elements_dont_match() {
    assert!(Ast::parse("<div></span><div></div>", false).is_err());
}
#[test]
fn it_can_parse_one_comment() {
    assert!(Ast::parse("<!-- hello !\"#/()= -->", false).is_ok());
}
#[test]
fn it_can_parse_multiple_comments() {
    assert!(Ast::parse("<!--x--><!--y--><!--z-->", false).is_ok());
}
#[test]
fn it_can_parse_one_text() {
    assert!(Ast::parse("hello world", false).is_ok());
}
#[test]
fn it_can_parse_multiple_rows_of_text() {
    assert!(Ast::parse("hello\nworld\n!", false).is_ok());
}
#[test]
fn it_can_parse_element_comment_text() {
    assert!(Ast::parse("<div></div><!--x-->hello", false).is_ok());
}
#[test]
fn it_can_parse_nested_elements() {
    assert!(Ast::parse("<div><div></div></div>", false).is_ok());
}
#[test]
fn it_can_parse_nested_elements_comments_text() {
    assert!(Ast::parse("<p id='body'><i>hello</i><!--x-->world</p>", false).is_ok());
}
#[test]
fn it_can_parse_nested_and_indented() {
    let markup = indoc!(
        r#"
            <p id='body'>
                <i>hello</i>
                <!--x-->
                world
            </p>
        "#
    );
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_deeply_nested() {
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
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_script_with_content() {
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
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_style_with_content() {
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
    assert!(Ast::parse(markup, false).is_ok());
}
