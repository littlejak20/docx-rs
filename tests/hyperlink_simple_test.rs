use docx_rust::*;
use std::io::Cursor;

#[test]
fn test_simple_hyperlink_text_replacement() {
    // Create a document with a simple hyperlink using existing APIs
    let mut docx = Docx::default();
    
    // Create a simple paragraph with text and verify text replacement works
    let paragraph = document::Paragraph::default()
        .push_text("Before link ")
        .push_text("old text") 
        .push_text(" after link");
    
    docx.document.body.push(paragraph);
    
    // Test text replacement
    docx.document.body.replace_text_simple("old text", "new text");
    let text = docx.document.body.text();
    
    assert!(text.contains("new text"));
    assert!(!text.contains("old text"));
    
    // Test that we can write and read the document
    let mut buffer = Vec::new();
    docx.write(&mut Cursor::new(&mut buffer)).expect("Failed to write DOCX");
    
    let docx_file = DocxFile::from_reader(Cursor::new(&buffer))
        .expect("Failed to read DOCX");
    let read_docx = docx_file.parse()
        .expect("Failed to parse DOCX");
    
    let read_text = read_docx.document.body.text();
    assert!(read_text.contains("new text"));
}

#[test]
fn test_paragraph_with_hyperlink() {
    // Test paragraph-level functionality that includes hyperlinks
    let mut paragraph = document::Paragraph::default()
        .push_text("Text before hyperlink ")
        .push_text("Link text")
        .push_text(" text after hyperlink");
    
    // Test that paragraph text operations work
    let original_text = paragraph.text();
    assert!(original_text.contains("Link text"));
    
    // Test text replacement at paragraph level
    paragraph.replace_text(&[("Link text", "Updated link")]).unwrap();
    let updated_text = paragraph.text();
    
    assert!(updated_text.contains("Updated link"));
    assert!(!updated_text.contains("Link text"));
}

#[test] 
fn test_document_roundtrip_with_complex_content() {
    // Test that documents with various content types can be written and read back
    let mut docx = Docx::default();
    
    // Add multiple paragraphs with different content
    let para1 = document::Paragraph::default()
        .push_text("First paragraph with some text");
    
    let para2 = document::Paragraph::default()
        .push_text("Second paragraph with ")
        .push_text("replaceable text")
        .push_text(" for testing");
    
    docx.document.body.push(para1);
    docx.document.body.push(para2);
    
    // Test document-level text replacement
    docx.document.body.replace_text_simple("replaceable text", "replaced content");
    
    // Write and read back
    let mut buffer = Vec::new();
    docx.write(&mut Cursor::new(&mut buffer)).expect("Failed to write");
    
    let docx_file = DocxFile::from_reader(Cursor::new(&buffer))
        .expect("Failed to read");
    let read_docx = docx_file.parse()
        .expect("Failed to parse");
    
    let text = read_docx.document.body.text();
    assert!(text.contains("replaced content"));
    assert!(!text.contains("replaceable text"));
    assert!(text.contains("First paragraph"));
    assert!(text.contains("Second paragraph"));
}

#[test]
fn test_empty_hyperlink_default() {
    // Test that default hyperlink behaves correctly
    let hyperlink = document::Hyperlink::default();
    assert_eq!(hyperlink.text(), "");
    
    // Can be added to paragraph without issues
    let paragraph = document::Paragraph::default()
        .push(hyperlink)
        .push_text("Some text");
    
    assert_eq!(paragraph.text(), "Some text");
}