use chrono::Local;
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::Browser;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Reads the contents of a file and returns it as a string.
///
/// # Arguments
///
/// * `path` - A string slice that represents the path to the file.
///
/// # Returns
///
/// A string containing the contents of the file.
pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("Failed to read file")
}

/// Adds `<pre>` tags to each line of the input text.
///
/// # Arguments
///
/// * `text` - A string slice that represents the input text.
///
/// # Returns
///
/// A string with `<pre>` tags added to each line of the input text.
pub fn add_pre_to_line(text: &str) -> String {
    let mut after = String::new();
    for i in text.lines() {
        after.push_str("<pre>");
        after.push_str(i);
        after.push_str("</pre>\n");
    }
    after
}

/// Saves the input text as an HTML file.
///
/// # Arguments
///
/// * `text` - A string slice that represents the input text.
/// * `path` - A string slice that represents the path to save the HTML file.
pub fn save_html(text: &str, path: &str) {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

/// Converts an HTML file to a PDF file.
///
/// # Arguments
///
/// * `path` - A string slice that represents the path to the HTML file.
pub fn html_to_pdf(path: &str) {
    let browser = Browser::default().expect("browser init err");
    let tab = browser.new_tab().expect("browser new tab err");

    // Navigate to the HTML file
    tab.navigate_to(path).expect("Navigate err");
    tab.wait_for_element("body").unwrap();

    let mut options = Some(PrintToPdfOptions::default());
    options.as_mut().unwrap().print_background = Some(true);
    options.as_mut().unwrap().margin_top = Some(0 as f64);
    options.as_mut().unwrap().margin_left = Some(0 as f64);
    options.as_mut().unwrap().margin_right = Some(0 as f64);
    options.as_mut().unwrap().margin_bottom = Some(0 as f64);

    match tab.print_to_pdf(options) {
        Err(_) => panic!("print_to_pdf err"),
        Ok(pdf_data) => {
            let mut file = File::create("./tmp.pdf").expect("create file err");
            file.write_all(&pdf_data).expect("write file err");
        }
    }
}

/// Adds the current date in the format "YYYY-MM-DD" to the output string.
///
/// # Returns
///
/// A string containing the current date in the format "YYYY-MM-DD".
pub fn add_date() -> String {
    let re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2}|\d{1})-(?P<day>\d{2}|\d{1})").unwrap();
    let now = Local::now().to_string();
    let caps = re.captures(&now).unwrap();
    match Some(caps) {
        Some(caps) => format!(
            "<div class='date'>{}-{}-{}</div>",
            &caps["year"], &caps["month"], &caps["day"]
        ),
        None => panic!("add_date err"),
    }
}
