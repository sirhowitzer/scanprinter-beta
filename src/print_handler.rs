extern crate printpdf;

use printpdf::*;
use std::collections::BTreeMap;
use std::fs::File;

pub fn save_template() {
    println!("Testing HTML to PDF implementation...");

    // Get HTML file from command line args
    let args: Vec<String> = std::env::args().collect();
    let html = if args.len() >= 2 {
        println!("Reading HTML from file: {}", args[1]);
        std::fs::read_to_string(&args[1]).expect("Failed to read HTML file")
    } else {
        // Default HTML content
        r#"
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Simple HTML Template</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            text-align: center;
            margin-top: 50px;
        }
        button {
            padding: 10px 20px;
            font-size: 16px;
            background-color: #007BFF;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
        }
        button:hover {
            background-color: #0056b3;
        }
    </style>
</head>
<body>
    <h1>Welcome to My Page</h1>
    <p>Click the button below:</p>
    <button onclick="alert('Button clicked!')">Click Me</button>
</body>
</html>
        "#.to_string()
    };

    // Create PDF from HTML
    let images = BTreeMap::new();
    let fonts = BTreeMap::new();
    let options = GeneratePdfOptions::default();
    let mut warnings = Vec::new();

    println!("Parsing HTML and generating PDF...");
    
    let doc = match PdfDocument::from_html(&html, &images, &fonts, &options, &mut warnings) {
        Ok(doc) => {
            println!("[OK] Successfully generated PDF");
            if !warnings.is_empty() {
                println!("Warnings:");
                for warn in &warnings {
                    println!("  - {:?}", warn);
                }
            }
            doc
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to generate PDF: {}", e);
            return;
        }
    };

    // Save to file
    let output_path = if args.len() >= 3 {
        args[2].clone()
    } else {
        "assets/html_example.pdf".to_string()
    };
    println!("Saving PDF to {}...", output_path);
    
    let save_options = PdfSaveOptions::default();
    let mut save_warnings = Vec::new();
    let bytes = doc.save(&save_options, &mut save_warnings);
    
    if !save_warnings.is_empty() {
        println!("Save warnings:");
        for warn in &save_warnings {
            println!("  - {:?}", warn);
        }
    }
    
    match File::create(&output_path) {
        Ok(mut file) => {
            use std::io::Write;
            match file.write_all(&bytes) {
                Ok(_) => println!("[OK] PDF saved successfully!"),
                Err(e) => eprintln!("[ERROR] Failed to write PDF: {}", e),
            }
        }
        Err(e) => eprintln!("[ERROR] Failed to create file: {}", e),
    }
}