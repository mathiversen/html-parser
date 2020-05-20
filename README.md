# Html parser

_WIP - work in progress, use at your own risk_

A simple and extensible html parser.

## How to use

```rust
    use html_parser::{HtmlParser, Result};

    fn main() -> Result<()> {
        let html = "
            <!doctype html>
            <html>
                <head>
                    <title>Html parser</title>
                </head>
                <body>
                    <h1>Hello world</h1>
                </body>
            </html>";

        let json = HtmlParser::parse(html)?.to_json_pretty()?;
        println!("{}", json);
        Ok(())
    }
```

## Library goal

The goal for this library is not to replace any of the top modern parsers that are currently used in popular browsers. If you're looking for something that is battle tested and used in serious projects, then I recommend you to have a look at https://github.com/servo/html5ever or similar. My initial goal with this project was to learn more about parsing and to build something useful. If you feel that the solution can be improved, please read more under contributions.

## How does it work?

In short:

- It Parses text with grammar defined in `grammar.pest`.
- It then translate the outcome of that grammar and builds an `AST` (Abstract Syntax Tree).
- The `AST` can be consumed as json

The library is highly dependant on the fantastic general purpose parser Pest, and you can find out more about this library at their official website https://pest.rs/ or at the documentation https://docs.rs/crate/pest/. If you want to know in detail how this parser is constructed then you should navigate to the `grammar.pest` file inside of the `src` directory.

I've tried to be explicit about each pair of rules but html is a very tricky protocol to parse, especially if you want to make it work with any website that you find on the web, since the protocol is very forgiving and many websites also include related protocols such as xhtml (html with xml-complient syntax) and css, svg, script-tags that include javascript etc.

## Contributions

I would love to get some feedback if you find my little project useful. Please feel free to highlight issues with my code or even submit a PR in case you think you can improve it.

## License

Copyright (c) 2020 Mathias Iversen

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
