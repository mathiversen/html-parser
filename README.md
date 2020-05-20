# Html parser

_WIP - work in progress_

This library aims to be a smal but functional html parser.

Even tough the name is `html parser` the goal (for now) is not to be a 100% complient html parser or replace any of the top modern parsers that are currently used in the popular browsers. If you're looking for something that is battle tested and used in serious projects, then I recommend you to have a look at https://github.com/servo/html5ever or similar. The goal was rather to learn how to build a parser/construct an ast and to build a lib that can be used by other programs.

## How to use

```rust
    use html_parser::HtmlParser;

    let html = "
        <html>
            <head>
                <title>Html parser</title>
            </head>
            <body>
                <h1>Hello world</h1>
            <body>
        </html>";

    let ast = HtmlParser::parse(html)?;
```

## How does it work?

In short:

- Parse text with grammar defined in `grammar.pest`.
- Translate the outcome of that grammar and build an `AST` (Abstract Syntax Tree).

The library is highly dependant on the fantastic general purpose parser Pest, and you can find out more about this library at their official website https://pest.rs/ or at the documentation https://docs.rs/crate/pest/. If you want to know in detail how this parser is constructed then you should navigate to the `grammar.pest` file inside of the `src` directory.

I've tried to be explicit about each pair of rules but html is a very tricky protocol to parse, especially if you want to make it work with any website that you find on the web, since the protocl is very forgiving and many websites also includes related protocols such as xhtml (html with xml-complient syntax) and css, svg, script-tags that include javascript etc.

Once the text has gone through the parser and constructed a tree with identified pairs it then goes through the process of constructing the `AST`. I've then constructed the tree in such a way that every identified part of the text gets classified into nodes, and there can be text-nodes and element-nodes. I decided to hide the comment-nodes from the `AST`, the motivation behind this is that I don't consider them as useful other than in the context of commentating on the html itself.

## Contributions

I would love to get some feedback if you find my little project useful. Please feel free to highlight issues with my code or even submit a PR in case you think you can improve it.

## MIT License

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
