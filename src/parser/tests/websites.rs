use crate::parser::HtmlParser;
use anyhow::Result;
use indoc::indoc;

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
async fn it_can_parse_mathias() -> Result<()> {
    let response = async_std::task::spawn(async {
        let url = "http://mathiasiversen.com/";
        surf::get(url)
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
async fn it_can_parse_spotify() -> Result<()> {
    let response = async_std::task::spawn(async {
        let bytesafe_url = "https://www.spotify.com/se";
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
async fn it_can_parse_bytesafe() -> Result<()> {
    let response = async_std::task::spawn(async {
        let url = "https://www.bytesafe.dev";
        surf::get(url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    let _x = HtmlParser::parse(&page, false);
    assert!(true);
    Ok(())
}

#[async_std::test]
async fn it_can_parse_amazon() -> Result<()> {
    let response = async_std::task::spawn(async {
        let url = "https://www.amazon.com/";
        surf::get(url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    let _x = HtmlParser::parse(&page, false)?;
    assert!(true);
    Ok(())
}
