use docx_rust::*;
use std::fs;
use std::io::Cursor;
use std::path::Path;

/// Detailed analysis of the DOCX sample files to understand their structure
#[test]
fn analyze_sample_docx_files() {
    let test_files = [
        ("docx/hyperlink/simple/hyperlink-simple.docx", "Simple Hyperlink"),
        ("docx/hyperlink/bigger/hyperlink-bigger.docx", "Bigger Hyperlink"),
        ("docx/toc/bigger/inhaltsverzeichnis-bigger.docx", "Bigger TOC"),
    ];

    for (file_path, description) in &test_files {
        if !Path::new(file_path).exists() {
            println!("⚠️  Skipping non-existent file: {}", file_path);
            continue;
        }
        
        println!("\n🔍 ANALYZING: {} ({})", file_path, description);
        println!("═══════════════════════════════════════════════════");
        
        let original_data = fs::read(file_path).expect("Failed to read file");
        let docx_file = DocxFile::from_reader(Cursor::new(&original_data))
            .expect("Failed to create DocxFile");
        let docx = docx_file.parse().expect("Failed to parse DOCX");
        
        // Basic stats
        let text = docx.document.body.text();
        println!("📄 Text length: {} characters", text.len());
        println!("📄 Paragraph count: {}", count_paragraphs(&docx));
        
        // Analyze paragraphs and hyperlinks
        let mut hyperlink_details = Vec::new();
        let mut paragraph_index = 0;
        
        for content in &docx.document.body.content {
            if let document::BodyContent::Paragraph(paragraph) = content {
                let para_text = paragraph.text();
                if !para_text.trim().is_empty() {
                    println!("📄 Paragraph {}: \"{}\"", paragraph_index, 
                            truncate_text(&para_text, 60));
                }
                
                // Analyze paragraph content
                for (i, para_content) in paragraph.content.iter().enumerate() {
                    match para_content {
                        document::ParagraphContent::Link(hyperlink) => {
                            let link_text = hyperlink.text();
                            let run_count = hyperlink.content.len();
                            
                            println!("  🔗 Hyperlink {}: \"{}\" ({} runs)", 
                                   i, truncate_text(&link_text, 40), run_count);
                            
                            if let Some(id) = &hyperlink.id {
                                println!("     ID: {}", id);
                            }
                            if let Some(anchor) = &hyperlink.anchor {
                                println!("     Anchor: {}", anchor);
                            }
                            
                            // Show each run in the hyperlink
                            for (run_idx, content) in hyperlink.content.iter().enumerate() {
                                match content {
                                    document::HyperlinkContent::Run(run) => {
                                        let run_text = run.text();
                                        if !run_text.is_empty() {
                                            println!("     Run {}: \"{}\"", run_idx, 
                                                    truncate_text(&run_text, 30));
                                        }
                                    }
                                }
                            }
                            
                            hyperlink_details.push((paragraph_index, i, link_text, run_count));
                        }
                        document::ParagraphContent::Run(run) => {
                            let run_text = run.text();
                            if !run_text.trim().is_empty() && run_text.len() > 10 {
                                println!("  📝 Run {}: \"{}\"", i, 
                                        truncate_text(&run_text, 40));
                            }
                        }
                        _ => {}
                    }
                }
                paragraph_index += 1;
            }
        }
        
        // Summary of hyperlinks
        if !hyperlink_details.is_empty() {
            println!("\n🔗 HYPERLINK SUMMARY:");
            for (para_idx, link_idx, text, run_count) in hyperlink_details {
                println!("  • P{}-L{}: \"{}\" ({} runs)", 
                        para_idx, link_idx, truncate_text(&text, 40), run_count);
            }
        } else {
            println!("\n🔗 No hyperlinks found (might be TOC with internal structure)");
        }
        
        // Show first few lines of text
        println!("\n📖 CONTENT PREVIEW:");
        let lines: Vec<&str> = text.lines().take(5).collect();
        for (i, line) in lines.iter().enumerate() {
            if !line.trim().is_empty() {
                println!("  {}: {}", i + 1, truncate_text(line, 70));
            }
        }
    }
}

fn count_paragraphs(docx: &Docx) -> usize {
    docx.document.body.content.iter()
        .filter(|content| matches!(content, document::BodyContent::Paragraph(_)))
        .count()
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}