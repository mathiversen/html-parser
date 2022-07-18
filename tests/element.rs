use html_parser::{Dom, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_one_element() -> Result<()> {
    let html = "<html></html>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_one_element_upper_case() -> Result<()> {
    let html = "<HTML></HTML>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case() -> Result<()> {
    let html = "<Html></Html>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case_numbers() -> Result<()> {
    let html = "<Header1></Header1>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case_numbers_symbols() -> Result<()> {
    let html = "<Head_Er-1></Head_Er-1>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_multiple_elements() -> Result<()> {
    let html = "<div/><div/>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_multiple_open_elements() -> Result<()> {
    let html = "<div></div><div></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_nested_elements() -> Result<()> {
    let html = indoc!(
        r"
        <div>
            <div />
        </div>
    "
    );
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_nested_elements_mixed_children() -> Result<()> {
    let html = indoc!(
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
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_deeply_nested() -> Result<()> {
    let html = indoc!(
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
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_script_with_content() -> Result<()> {
    let html = indoc!(
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
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_style_with_content() -> Result<()> {
    let html = indoc!(
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
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_skips_dangling_elements() -> Result<()> {
    let html = indoc!(
        "
        <div id='123'></div>
        </div>
        <div id='321'></div>
    "
    );
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_broken_html() -> Result<()> {
    let html = "<div></span><div></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_errors_when_multiple_nested_elements_dont_match() -> Result<()> {
    let html = "<div><div><div><div></div></div_error></div></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_clone_node() {
    let html = indoc!(
        "
        <div>one</div>
        <div>two</div>
    "
    );
    let dom = Dom::parse(html).unwrap();
    let one = dom.children[0].clone();
    assert_json_snapshot!(one);
}
#[test]
fn it_can_clone_dom() {
    let html = indoc!(
        "
        <html>
            <head>
                <title>Title</title>
            </head>
            <body>
                <h1>Hello world</h1>
            </body>
        </html>
    "
    );
    let dom = Dom::parse(html).unwrap();
    let dom_clone = dom.clone();
    assert_eq!(dom, dom_clone);
}

#[test]
fn it_can_deal_with_weird_whitespaces() {
    let html = indoc!(
        "
        <!-- Normal case -->
        <div> Text </div>

        <!-- Whitespaces in opening tag to the left -->
        < div> Text </div>

        <!-- Whitespaces in opening tag to the right -->
        <div > Text </div>

        <!-- Whitespaces in closing tag to the left (should not work) -->
        <div> Text < /div>

        <!-- Whitespaces in closing tag to the right -->
        <div> Text </div >

        <!-- Whitespaces everywhere (should not work) -->
        < div > Text < / div >
        "
    );
    let dom = Dom::parse(html).unwrap();
    assert_json_snapshot!(dom);
}
