use html_parser::Dom;
use indoc::indoc;

#[test]
fn it_can_parse_simple() {
    let html = indoc!(
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
    assert!(Dom::parse(html).is_ok());
}

#[test]
fn it_can_parse_spotify() {
    let resp = reqwest::blocking::get("https://www.spotify.com/se")
        .unwrap()
        .text()
        .unwrap();
    assert!(Dom::parse(&resp).is_ok());
}

#[ignore]
#[test]
fn it_can_parse_facebook() {
    let resp = reqwest::blocking::get("https://www.facebook.com/")
        .unwrap()
        .text()
        .unwrap();
    assert!(Dom::parse(&resp).is_ok());
}

#[ignore]
#[test]
fn it_can_parse_amazon() {
    let resp = reqwest::blocking::get("https://www.amazon.com/")
        .unwrap()
        .text()
        .unwrap();
    assert!(Dom::parse(&resp).is_ok());
}

#[ignore]
#[test]
fn it_can_parse_apple() {
    let resp = reqwest::blocking::get("https://www.apple.com/")
        .unwrap()
        .text()
        .unwrap();
    assert!(Dom::parse(&resp).is_ok());
}

#[ignore]
#[test]
fn it_can_parse_nytimes() {
    let resp = reqwest::blocking::get("https://www.nytimes.com/")
        .unwrap()
        .text()
        .unwrap();
    assert!(Dom::parse(&resp).is_ok());
}

#[ignore]
#[test]
fn it_can_parse_wikipedia() {
    let resp = reqwest::blocking::get("https://en.wikipedia.org/wiki/Main_Page")
        .unwrap()
        .text()
        .unwrap();
    assert!(Dom::parse(&resp).is_ok());
}
