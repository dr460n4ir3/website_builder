use std::io;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum HtmlElement {
    Title(String),
    Heading1(String),
    Heading2(String),
    Paragraph(String),
}

impl HtmlElement {
    fn render(&self) -> String {
        match self {
            HtmlElement::Title(value) => format!("<title>{}</title>", value),
            HtmlElement::Heading1(value) => format!("<h1>{}</h1>", value),
            HtmlElement::Heading2(value) => format!("<h2>{}</h2>", value),
            HtmlElement::Paragraph(value) => format!("<p>{}</p>", value),
        }
    }
}

// NOTE: need to fix this function!
fn update_html() -> bool {
    let mut update = String::new();
    println!("Do you want to update the html file? (y/n)");
    io::stdin()
        .read_line(&mut update)
        .expect("Failed to read line");
    let update = update.trim();
    if update == "y" {
        return true;
    } else {
        return false;
    }
}

// create a function to write the `full_html` string to a file
fn write_html_to_file(full_html: &str) {
    let mut file = match File::create("index.html") {
        Ok(file) => file,
        Err(err) => {
            println!("Could not create file: {}", err);
            return;
        }
    };

    match file.write_all(full_html.as_bytes()) {
        Ok(_) => println!("HTML written to file"),
        Err(err) => println!("Could not write to file: {}", err),
    };
}


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
    write_html_to_file(&full_html);

    let should_update = update_html();

    if should_update {
        println!("Updating html file...");
    } else {
        println!("Not updating html file...");
    }

    if should_update {
        write_html_to_file(&full_html);
    }

}

        