use crate::parser::HtmlParser;
use anyhow::Result;
use indoc::indoc;

#[test]
fn it_can_parse_empty_document() -> Result<()> {
    assert_eq!((), HtmlParser::parse("", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_element() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<html></html>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_element_upper_case() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<HTML></HTML>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<Html></Html>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case_numbers() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<Header1></Header1>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_element_mixed_case_numbers_symbols() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<Head_er-1></Head_er-1>", false)?);
    Ok(())
}
#[test]
fn it_errors_when_case_dont_match() -> Result<()> {
    assert!(HtmlParser::parse("<html></Html>", false).is_err());
    Ok(())
}
#[test]
fn it_errors_when_element_name_dont_match() -> Result<()> {
    assert!(HtmlParser::parse("<html></div>", false).is_err());
    Ok(())
}
#[test]
fn it_can_parse_multiple_elements() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<div></div><div></div>", false)?);
    Ok(())
}
#[test]
fn it_errors_when_multiple_elements_dont_match() -> Result<()> {
    assert!(HtmlParser::parse("<div></span><div></div>", false).is_err());
    Ok(())
}
#[test]
fn it_can_parse_one_comment() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<!-- hello !\"#/()= -->", false)?);
    Ok(())
}
#[test]
fn it_can_parse_multiple_comments() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<!--x--><!--y--><!--z-->", false)?);
    Ok(())
}
#[test]
fn it_can_parse_one_text() -> Result<()> {
    assert_eq!((), HtmlParser::parse("hello world", false)?);
    Ok(())
}
#[test]
fn it_can_parse_multiple_rows_of_text() -> Result<()> {
    assert_eq!((), HtmlParser::parse("hello\nworld\n!", false)?);
    Ok(())
}
#[test]
fn it_can_parse_element_comment_text() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<div></div><!--x-->hello", false)?);
    Ok(())
}
#[test]
fn it_can_parse_nested_elements() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<div><div></div></div>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_nested_elements_comments_text() -> Result<()> {
    assert_eq!(
        (),
        HtmlParser::parse("<p id='body'><i>hello</i><!--x-->world</p>", false)?
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
    assert_eq!((), HtmlParser::parse(markup, false)?);
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
    assert_eq!((), HtmlParser::parse(markup, false)?);
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
    assert_eq!((), HtmlParser::parse(markup, false)?);
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
    assert_eq!((), HtmlParser::parse(markup, false)?);
    Ok(())
}
#[test]
fn it_can_parse_simple_html_page() -> Result<()> {
    let markup = indoc!(
        r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Document</title>
                    <style>
                        body {
                            background: black;
                        }
                
                        h1 {
                            color: white;
                        }
                    </style>
                </head>
                <body>
                    <h1>Hello world</h1>
                    <!-- There should be more text here -->
                    <script>
                        const title = document.querySelector("h1")
                        title.innerText = "Hello from script"
                    </script>
                </body>
            </html>        
        "#
    );
    assert_eq!((), HtmlParser::parse(markup, false)?);
    Ok(())
}

#[async_std::test]
async fn it_can_parse_gill() -> Result<()> {
    let response = async_std::task::spawn(async {
        let bytesafe_url = "https://gill.net.in/";
        surf::get(bytesafe_url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    let _x = HtmlParser::parse(&page, false)?;
    assert!(true);
    Ok(())
}

#[async_std::test]
async fn it_can_parse_facebook() -> Result<()> {
    let response = async_std::task::spawn(async {
        let bytesafe_url = "https://www.facebook.com/";
        surf::get(bytesafe_url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    let _x = HtmlParser::parse(&page, false)?;
    assert!(true);
    Ok(())
}

#[async_std::test]
async fn it_can_parse_mathias() -> Result<()> {
    let response = async_std::task::spawn(async {
        let mathias_url = "http://mathiasiversen.com/";
        surf::get(mathias_url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    let _x = HtmlParser::parse(&page, false)?;
    assert!(true);
    Ok(())
}

#[async_std::test]
async fn it_can_parse_jamiller() -> Result<()> {
    let response = async_std::task::spawn(async {
        let mathias_url = "https://jamiller.me/";
        surf::get(mathias_url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    let _x = HtmlParser::parse(&page, false)?;
    assert!(true);
    Ok(())
}

#[async_std::test]
async fn it_can_parse_bytesafe() -> Result<()> {
    let response = async_std::task::spawn(async {
        let mathias_url = "https://www.spotify.com/se/";
        surf::get(mathias_url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    let _x = HtmlParser::parse(&page, false)?;
    assert!(true);
    Ok(())
}

#[async_std::test]
async fn it_can_parse_amazon() -> Result<()> {
    let response = async_std::task::spawn(async {
        let mathias_url = "https://www.amazon.com/";
        surf::get(mathias_url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    let _x = HtmlParser::parse(&page, false)?;
    assert!(true);
    Ok(())
}
