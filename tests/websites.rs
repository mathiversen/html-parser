use async_std;
use html_parser::Dom;
use indoc::indoc;
use surf;

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

#[async_std::test]
async fn it_can_parse_spotify() {
    let response = async_std::task::spawn(async {
        let bytesafe_url = "https://www.spotify.com/se";
        surf::get(bytesafe_url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    assert!(Dom::parse(&page).is_ok());
}

#[async_std::test]
async fn it_can_parse_facebook() {
    let response = async_std::task::spawn(async {
        let bytesafe_url = "https://www.facebook.com/";
        surf::get(bytesafe_url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    assert!(Dom::parse(&page).is_ok());
}

#[async_std::test]
async fn it_can_parse_amazon() {
    let response = async_std::task::spawn(async {
        let url = "https://www.amazon.com/";
        surf::get(url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    assert!(Dom::parse(&page).is_ok());
}

#[ignore]
#[async_std::test]
async fn it_can_parse_apple() {
    let response = async_std::task::spawn(async {
        let url = "https://www.apple.com/";
        surf::get(url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    assert!(Dom::parse(&page).is_ok());
}

#[ignore]
#[async_std::test]
async fn it_can_parse_nytimes() {
    let response = async_std::task::spawn(async {
        let url = "https://www.nytimes.com/";
        surf::get(url)
            .recv_string()
            .await
            .expect("Could not get site")
    });
    let page = response.await;
    assert!(Dom::parse(&page).is_ok());
}
