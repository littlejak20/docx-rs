use docx_rust::*;
use std::fs;
use std::io::Cursor;
use std::path::Path;

/// Test all sample DOCX files and generate output files for manual verification in Microsoft Word
#[test]
fn test_and_output_all_sample_docx_files() {
    let test_files = [
        // Simple hyperlink files
        ("docx/hyperlink/simple/hyperlink-simple.docx", "output/hyperlink-simple-processed.docx"),
        
        // Bigger hyperlink files
        ("docx/hyperlink/bigger/hyperlink-bigger.docx", "output/hyperlink-bigger-processed.docx"),
        ("docx/hyperlink/bigger/hyperlink-bigger-anonymisiert.docx", "output/hyperlink-bigger-anon-processed.docx"),
        
        // Simple table of contents files
        ("docx/toc/simple/inhaltsverzeichnis-simple-anonymisiert.docx", "output/toc-simple-anon-processed.docx"),
        
        // Bigger table of contents files  
        ("docx/toc/bigger/inhaltsverzeichnis-bigger.docx", "output/toc-bigger-processed.docx"),
        ("docx/toc/bigger/inhaltsverzeichnis-bigger-anonymisiert.docx", "output/toc-bigger-anon-processed.docx"),
    ];

    // Create output directory
    fs::create_dir_all("output").expect("Failed to create output directory");
    
    let mut test_results = Vec::new();
    println!("\n🔄 PROCESSING DOCX FILES WITH HYPERLINK FIX:");
    println!("═══════════════════════════════════════════════════");
    
    for (input_path, output_path) in &test_files {
        println!("\n📄 Processing: {}", input_path);
        
        match process_and_output_docx_file(input_path, output_path) {
            Ok(stats) => {
                println!("✅ SUCCESS: {} -> {}", input_path, output_path);
                println!("   📊 {}", stats);
                test_results.push((input_path, output_path, true, stats));
            }
            Err(e) => {
                println!("❌ ERROR: {} - {}", input_path, e);
                test_results.push((input_path, output_path, false, format!("Error: {}", e)));
            }
        }
    }
    
    // Print comprehensive summary
    println!("\n📊 PROCESSING SUMMARY:");
    println!("═══════════════════════════════════════════════════");
    let successful = test_results.iter().filter(|(_, _, success, _)| *success).count();
    let total = test_results.len();
    
    for (input, output, success, stats) in &test_results {
        let status = if *success { "✅" } else { "❌" };
        println!("{} {} -> {}", status, input, output);
        println!("   {}", stats);
    }
    
    println!("\n🎯 FINAL RESULT: {}/{} files processed successfully", successful, total);
    
    if successful > 0 {
        println!("\n📁 OUTPUT FILES CREATED:");
        println!("   Check the 'output/' directory for processed DOCX files");
        println!("   These can be opened manually in Microsoft Word to verify functionality");
    }
    
    // Fail the test if any file failed
    if successful != total {
        panic!("Some DOCX files failed to process correctly");
    }
}

/// Process a single DOCX file with text replacements and output for manual verification
fn process_and_output_docx_file(input_path: &str, output_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Check if input file exists
    if !Path::new(input_path).exists() {
        return Err(format!("Input file not found: {}", input_path).into());
    }
    
    // Read the original file
    let original_data = fs::read(input_path)?;
    
    // Parse the DOCX file
    let docx_file = DocxFile::from_reader(Cursor::new(&original_data))?;
    let mut docx = docx_file.parse()?;
    
    // Extract original statistics
    let original_text = docx.document.body.text();
    let original_text_len = original_text.len();
    let paragraph_count = count_paragraphs(&docx);
    let hyperlink_count = count_hyperlinks(&docx);
    
    // Perform meaningful text replacements to test hyperlink functionality
    let replacements = [
        // German to English translations (common in TOC)
        ("Kapitel", "Chapter"),
        ("Überschrift", "Heading"), 
        ("Inhaltsverzeichnis", "Table of Contents"),
        ("Seite", "Page"),
        
        // Test number replacements
        ("1", "ONE"),
        ("2", "TWO"),
        
        // Test common words
        ("Text", "Content"),
        ("hier", "here"),
        ("ist", "is"),
        
        // Test hyperlink-specific content
        ("Textmarke", "Bookmark"),
        ("Link", "Hyperlink"),
    ];
    
    let mut replacement_count = 0;
    for (old, new) in &replacements {
        if original_text.contains(old) {
            docx.document.body.replace_text_simple(old, new);
            replacement_count += 1;
        }
    }
    
    // Get final text after replacements
    let final_text = docx.document.body.text();
    let final_text_len = final_text.len();
    
    // Write the processed document to output file
    docx.write_file(output_path)?;
    
    // Verify the output can be read back (integrity check)
    let verification_docx_file = DocxFile::from_file(output_path)?;
    let verification_docx = verification_docx_file.parse()?;
    let verification_text = verification_docx.document.body.text();
    
    // Verify text integrity
    if verification_text.len() != final_text_len {
        return Err(format!(
            "Text integrity check failed: expected {} chars, got {}", 
            final_text_len, 
            verification_text.len()
        ).into());
    }
    
    Ok(format!(
        "Paragraphs: {}, Hyperlinks: {}, Text: {} -> {} chars, Replacements: {}", 
        paragraph_count, 
        hyperlink_count, 
        original_text_len,
        final_text_len,
        replacement_count
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
fn test_specific_hyperlink_scenarios() {
    println!("\n🔬 TESTING SPECIFIC HYPERLINK SCENARIOS:");
    
    let scenarios = [
        ("docx/hyperlink/simple/hyperlink-simple.docx", "Simple URL hyperlink"),
        ("docx/hyperlink/bigger/hyperlink-bigger.docx", "Multi-run hyperlinks with anchors"),
    ];
    
    for (file_path, description) in &scenarios {
        if !Path::new(file_path).exists() {
            println!("⚠️  Skipping non-existent file: {}", file_path);
            continue;
        }
        
        println!("\n📋 Testing: {} ({})", file_path, description);
        
        // Read and analyze the file
        let original_data = fs::read(file_path).expect("Failed to read file");
        let docx_file = DocxFile::from_reader(Cursor::new(&original_data))
            .expect("Failed to create DocxFile");
        let mut docx = docx_file.parse().expect("Failed to parse DOCX");
        
        // Analyze hyperlinks before processing
        let mut hyperlink_details = Vec::new();
        for content in &docx.document.body.content {
            if let document::BodyContent::Paragraph(paragraph) = content {
                for para_content in &paragraph.content {
                    if let document::ParagraphContent::Link(hyperlink) = para_content {
                        let text = hyperlink.text();
                        let run_count = hyperlink.content.len();
                        let has_id = hyperlink.id.is_some();
                        let has_anchor = hyperlink.anchor.is_some();
                        
                        hyperlink_details.push((text, run_count, has_id, has_anchor));
                    }
                }
            }
        }
        
        // Report findings
        for (i, (text, runs, has_id, has_anchor)) in hyperlink_details.iter().enumerate() {
            println!("  🔗 Hyperlink {}: \"{}\" ({} runs, ID: {}, Anchor: {})", 
                   i + 1, 
                   truncate_text(text, 30), 
                   runs, 
                   has_id, 
                   has_anchor);
        }
        
        // Test text replacement specifically on hyperlinks
        if !hyperlink_details.is_empty() {
            docx.document.body.replace_text_simple("Text", "CONTENT");
            
            // Verify document can still be written
            let output_path = format!("output/test-{}", 
                file_path.split('/').last().unwrap_or("unknown.docx"));
            docx.write_file(&output_path).expect("Failed to write test output");
            
            println!("  ✅ Hyperlink processing successful, output: {}", output_path);
        }
    }
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}