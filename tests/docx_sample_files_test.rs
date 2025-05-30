use docx_rust::*;
use std::fs;
use std::io::Cursor;
use std::path::Path;

/// Test all sample DOCX files in the docx/ folder to ensure they can be processed
/// without corruption after the hyperlink fix.
#[test]
fn test_all_sample_docx_files() {
    let test_files = [
        // Simple hyperlink files
        "docx/hyperlink/simple/hyperlink-simple.docx",
        
        // Bigger hyperlink files
        "docx/hyperlink/bigger/hyperlink-bigger.docx",
        "docx/hyperlink/bigger/hyperlink-bigger-anonymisiert.docx",
        
        // Simple table of contents files
        "docx/toc/simple/inhaltsverzeichnis-simple-anonymisiert.docx",
        
        // Bigger table of contents files  
        "docx/toc/bigger/inhaltsverzeichnis-bigger.docx",
        "docx/toc/bigger/inhaltsverzeichnis-bigger-anonymisiert.docx",
    ];

    let mut test_results = Vec::new();
    
    for file_path in &test_files {
        println!("Testing file: {}", file_path);
        
        match test_single_docx_file(file_path) {
            Ok(stats) => {
                println!("✅ {} - SUCCESS: {}", file_path, stats);
                test_results.push((file_path, true, stats));
            }
            Err(e) => {
                println!("❌ {} - ERROR: {}", file_path, e);
                test_results.push((file_path, false, format!("Error: {}", e)));
            }
        }
    }
    
    // Print summary
    println!("\n📊 TEST SUMMARY:");
    let successful = test_results.iter().filter(|(_, success, _)| *success).count();
    let total = test_results.len();
    
    for (file, success, stats) in &test_results {
        let status = if *success { "✅" } else { "❌" };
        println!("{} {}: {}", status, file, stats);
    }
    
    println!("\n🎯 RESULT: {}/{} files processed successfully", successful, total);
    
    // Fail the test if any file failed
    if successful != total {
        panic!("Some DOCX files failed to process correctly");
    }
}

/// Test a single DOCX file by reading, processing, and writing it back
fn test_single_docx_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Check if file exists
    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path).into());
    }
    
    // Read the original file
    let original_data = fs::read(file_path)?;
    
    // Parse the DOCX file
    let docx_file = DocxFile::from_reader(Cursor::new(&original_data))?;
    let mut docx = docx_file.parse()?;
    
    // Extract text to verify content is readable
    let original_text = docx.document.body.text();
    let original_text_len = original_text.len();
    
    // Count paragraphs and hyperlinks for verification
    let paragraph_count = count_paragraphs(&docx);
    let hyperlink_count = count_hyperlinks(&docx);
    
    // Test text replacement to verify hyperlinks work correctly
    if hyperlink_count > 0 {
        // Perform a safe text replacement that shouldn't break anything
        docx.document.body.replace_text_simple("test", "test");
    }
    
    // Write the document back to memory
    let mut output_buffer = Vec::new();
    docx.write(&mut Cursor::new(&mut output_buffer))?;
    
    // Verify the output can be read back
    let verification_docx_file = DocxFile::from_reader(Cursor::new(&output_buffer))?;
    let verification_docx = verification_docx_file.parse()?;
    let final_text = verification_docx.document.body.text();
    
    // Verify text length is preserved (should be identical for safe replacement)
    if final_text.len() != original_text_len {
        return Err(format!(
            "Text length changed: {} -> {}", 
            original_text_len, 
            final_text.len()
        ).into());
    }
    
    Ok(format!(
        "Paragraphs: {}, Hyperlinks: {}, Text length: {}", 
        paragraph_count, 
        hyperlink_count, 
        original_text_len
    ))
}

/// Count paragraphs in the document
fn count_paragraphs(docx: &Docx) -> usize {
    docx.document.body.content.iter()
        .filter(|content| matches!(content, document::BodyContent::Paragraph(_)))
        .count()
}

/// Count hyperlinks in the document
fn count_hyperlinks(docx: &Docx) -> usize {
    let mut count = 0;
    
    for content in &docx.document.body.content {
        if let document::BodyContent::Paragraph(paragraph) = content {
            for para_content in &paragraph.content {
                if matches!(para_content, document::ParagraphContent::Link(_)) {
                    count += 1;
                }
            }
        }
    }
    
    count
}

#[test]
fn test_hyperlink_text_replacement_in_sample_files() {
    // Test more complex text replacement scenarios with sample files
    let test_files = [
        "docx/hyperlink/simple/hyperlink-simple.docx",
        "docx/toc/simple/inhaltsverzeichnis-simple-anonymisiert.docx",
    ];

    for file_path in &test_files {
        if !Path::new(file_path).exists() {
            println!("⚠️  Skipping non-existent file: {}", file_path);
            continue;
        }
        
        println!("Testing text replacement in: {}", file_path);
        
        // Read and parse file
        let original_data = fs::read(file_path).expect("Failed to read file");
        let docx_file = DocxFile::from_reader(Cursor::new(&original_data))
            .expect("Failed to create DocxFile");
        let mut docx = docx_file.parse().expect("Failed to parse DOCX");
        
        // Get original text
        let original_text = docx.document.body.text();
        println!("Original text length: {}", original_text.len());
        
        // Perform text replacement (using common German words that might be in TOC)
        let replacements = [
            ("Kapitel", "Chapter"),  // German to English
            ("Seite", "Page"),       // German to English
            ("1", "ONE"),            // Number replacement
        ];
        
        for (old, new) in &replacements {
            docx.document.body.replace_text_simple(old, new);
        }
        
        // Verify document can still be written and read
        let mut output_buffer = Vec::new();
        docx.write(&mut Cursor::new(&mut output_buffer))
            .expect("Failed to write modified DOCX");
        
        let verification_docx_file = DocxFile::from_reader(Cursor::new(&output_buffer))
            .expect("Failed to read back modified DOCX");
        let verification_docx = verification_docx_file.parse()
            .expect("Failed to parse modified DOCX");
        
        let final_text = verification_docx.document.body.text();
        println!("Final text length: {}", final_text.len());
        
        // Verify replacements were made
        for (old, new) in &replacements {
            if original_text.contains(old) {
                assert!(
                    final_text.contains(new), 
                    "Replacement '{}' -> '{}' not found in final text", 
                    old, 
                    new
                );
                assert!(
                    !final_text.contains(old),
                    "Original text '{}' still found after replacement",
                    old
                );
            }
        }
        
        println!("✅ Text replacement test passed for {}", file_path);
    }
}