use std::io;

#[derive(Debug)]
enum HtmlElement {
    Doctype,
    Title(String),
    Heading1(String),
    Heading2(String),
    Paragraph(String),
}

impl HtmlElement {
    fn render(&self) -> String {
        match self {
            HtmlElement::Doctype => String::from("<!DOCTYPE html>"),
            HtmlElement::Title(value) => format!("<title>{}</title>", value),
            HtmlElement::Heading1(value) => format!("<h1>{}</h1>", value),
            HtmlElement::Heading2(value) => format!("<h2>{}</h2>", value),
            HtmlElement::Paragraph(value) => format!("<p>{}</p>", value),
        }
    }
}

// write a function that will input text for the values of <title>, <h1>, <h2>,<p> tags and return a string of html
fn main() {
    let mut title = String::new();
    let mut h1 = String::new();
    let mut h2 = String::new();
    let mut p = String::new();

    println!("Enter title: ");
    io::stdin().read_line(&mut title).expect("Failed to read line");
    println!("Enter h1: ");
    io::stdin().read_line(&mut h1).expect("Failed to read line");
    println!("Enter h2: ");
    io::stdin().read_line(&mut h2).expect("Failed to read line");
    println!("Enter p: ");
    io::stdin().read_line(&mut p).expect("Failed to read line");

    let html_elements = vec![
        HtmlElement::Title(title),
        HtmlElement::Heading1(h1),
        HtmlElement::Heading2(h2),
        HtmlElement::Paragraph(p),
    ];

    // let html = html_elements
    //     .iter()
    //     .map(|element| element.render())
    //     .collect::<Vec<String>>()
    //     .join("\n");

    let full_html = format!(
        "<!DOCTYPE html>
<html lang='en'>
<head>
    <meta charset='UTF-8'>
    <meta http-equiv='X-UA-Compatible' content='IE=edge'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
{}
</head>
<body>
{}
{}
{}
</body>
</html>",
        html_elements[0].render(),
        html_elements[1].render(),
        html_elements[2].render(),
        html_elements[3].render(),
    );

    println!("");
    println!("{}", full_html);
    println!("");
}