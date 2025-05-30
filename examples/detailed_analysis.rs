use docx_rust::*;
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 DETAILLIERTE DOCX-STRUKTUR-ANALYSE");
    println!("=====================================");
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("📖 Verwendung: cargo run --example detailed_analysis DATEINAME.docx");
        return Ok(());
    }
    
    let input_file = &args[1];
    
    if !Path::new(input_file).exists() {
        println!("❌ Datei '{}' nicht gefunden", input_file);
        return Ok(());
    }
    
    println!("📄 Analysiere: {}", input_file);
    
    let docx_file = DocxFile::from_file(input_file)?;
    let docx = docx_file.parse()?;
    
    println!("\n📊 ÜBERSICHT:");
    println!("═════════════");
    let text = docx.document.body.text();
    println!("📝 Text-Länge: {} Zeichen", text.len());
    println!("📄 Paragraphen: {}", docx.document.body.content.len());
    
    // Detaillierte Analyse jedes Paragraphen
    println!("\n🔍 PARAGRAPH-ANALYSE:");
    println!("═══════════════════════");
    
    let mut total_hyperlinks = 0;
    let mut total_runs = 0;
    let mut total_field_chars = 0;
    let mut total_instr_text = 0;
    
    for (para_idx, content) in docx.document.body.content.iter().enumerate() {
        match content {
            document::BodyContent::Paragraph(paragraph) => {
                let para_text = paragraph.text();
                if !para_text.trim().is_empty() {
                    println!("\n📄 Paragraph {}: \"{}\"", 
                            para_idx + 1, 
                            truncate_text(&para_text, 60));
                }
                
                // Analysiere jeden Paragraph-Inhalt
                for (content_idx, para_content) in paragraph.content.iter().enumerate() {
                    match para_content {
                        document::ParagraphContent::Link(hyperlink) => {
                            total_hyperlinks += 1;
                            let link_text = hyperlink.text();
                            let run_count = hyperlink.content.len();
                            
                            println!("   🔗 Hyperlink {}: \"{}\" ({} runs)", 
                                   content_idx + 1, 
                                   truncate_text(&link_text, 40), 
                                   run_count);
                            
                            if let Some(id) = &hyperlink.id {
                                println!("      🆔 ID: {}", id);
                            }
                            if let Some(anchor) = &hyperlink.anchor {
                                println!("      ⚓ Anchor: {}", anchor);
                            }
                            
                            // Multi-Run Details
                            if run_count > 1 {
                                println!("      🎯 Multi-Run Hyperlink!");
                                for (run_idx, run) in hyperlink.content.iter().enumerate() {
                                    let run_text = run.text();
                                    if !run_text.is_empty() {
                                        println!("         Run {}: \"{}\"", 
                                                run_idx + 1, 
                                                truncate_text(&run_text, 20));
                                    }
                                }
                            }
                        }
                        document::ParagraphContent::Run(run) => {
                            total_runs += 1;
                            let run_text = run.text();
                            
                            // Analysiere Run-Inhalt auf Field-Codes
                            let mut has_field_char = false;
                            let mut has_instr_text = false;
                            
                            for run_content in &run.content {
                                match run_content {
                                    document::RunContent::FieldChar(_) => {
                                        has_field_char = true;
                                        total_field_chars += 1;
                                    }
                                    document::RunContent::InstrText(_) => {
                                        has_instr_text = true;
                                        total_instr_text += 1;
                                    }
                                    _ => {}
                                }
                            }
                            
                            // Zeige interessante Runs
                            if has_field_char || has_instr_text || run_text.contains("TOC") || 
                               run_text.contains("PAGEREF") || run_text.contains("_Toc") {
                                println!("   📝 Run {}: \"{}\"", 
                                        content_idx + 1, 
                                        truncate_text(&run_text, 40));
                                
                                if has_field_char {
                                    println!("      🔧 FieldChar gefunden (TOC-Struktur!)");
                                }
                                if has_instr_text {
                                    println!("      📄 InstrText gefunden (Field-Code!)");
                                }
                                if run_text.contains("TOC") {
                                    println!("      📋 TOC-Feld erkannt!");
                                }
                                if run_text.contains("PAGEREF") {
                                    println!("      🔗 PAGEREF-Verweis erkannt!");
                                }
                            }
                        }
                        _ => {
                            // Andere Inhaltstypen
                            println!("   📌 Anderer Inhalt: {:?}", 
                                    std::mem::discriminant(para_content));
                        }
                    }
                }
            }
            document::BodyContent::Table(table) => {
                println!("\n📋 Tabelle gefunden");
                // Hier könnten wir auch Tabellen analysieren
            }
            _ => {
                println!("\n📌 Anderer Body-Inhalt: {:?}", 
                        std::mem::discriminant(content));
            }
        }
    }
    
    // Zusammenfassung
    println!("\n📊 ZUSAMMENFASSUNG:");
    println!("═══════════════════");
    println!("🔗 Normale Hyperlinks: {}", total_hyperlinks);
    println!("📝 Runs gesamt: {}", total_runs);
    println!("🔧 FieldChar-Elemente: {}", total_field_chars);
    println!("📄 InstrText-Elemente: {}", total_instr_text);
    
    // Vermutung über Dokumenttyp
    println!("\n🎯 DOKUMENTTYP-ANALYSE:");
    println!("═══════════════════════");
    
    if total_hyperlinks > 0 {
        println!("✅ Dokument enthält normale Hyperlinks");
    }
    
    if total_field_chars > 0 || total_instr_text > 0 {
        println!("📋 Dokument enthält Field-Codes (wahrscheinlich TOC)");
        println!("   💡 Diese werden nicht als normale Hyperlinks gezählt!");
    }
    
    if text.contains("TOC") || text.contains("PAGEREF") || text.contains("_Toc") {
        println!("📖 Inhaltsverzeichnis-Struktur erkannt");
        println!("   💡 TOC-Hyperlinks sind in Field-Codes versteckt");
    }
    
    if total_hyperlinks == 0 && (total_field_chars > 0 || text.contains("TOC")) {
        println!("❓ Das erklärt warum 'Hyperlinks: 0' angezeigt wird:");
        println!("   📋 TOC verwendet Field-Codes statt normale Hyperlinks");
        println!("   🔧 Diese sind schwerer zu erkennen und zu verarbeiten");
    }
    
    // Text-Vorschau
    println!("\n📖 TEXT-VORSCHAU:");
    println!("═════════════════");
    let lines: Vec<&str> = text.lines().take(10).collect();
    for (i, line) in lines.iter().enumerate() {
        if !line.trim().is_empty() {
            println!("   {}: {}", i + 1, truncate_text(line, 70));
        }
    }
    
    Ok(())
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}