use docx_rust::*;
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 DOCX Hyperlink-Fix Tester");
    println!("=============================");
    
    // Argumente von Kommandozeile lesen
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("❌ Fehler: Kein Dateiname angegeben");
        println!("📖 Verwendung: cargo run --example test_docx DATEINAME.docx [--replace]");
        println!("📖 Beispiel: cargo run --example test_docx test.docx");
        println!("📖 Mit Text-Ersetzung: cargo run --example test_docx test.docx --replace");
        return Ok(());
    }
    
    // Prüfe ob Text-Ersetzung aktiviert werden soll
    let enable_text_replacement = args.len() > 2 && args[2] == "--replace";
    
    let input_file = &args[1];
    
    // Prüfe ob Datei existiert
    if !Path::new(input_file).exists() {
        println!("❌ Fehler: Datei '{}' nicht gefunden", input_file);
        println!("💡 Tipp: Kopiere deine DOCX-Datei in den docx-rs Ordner");
        return Ok(());
    }
    
    let output_file = format!("{}-processed.docx", 
        input_file.strip_suffix(".docx").unwrap_or(input_file));
    
    println!("📄 Lade Datei: {}", input_file);
    
    // DOCX laden und parsen
    let docx_file = match DocxFile::from_file(input_file) {
        Ok(file) => {
            println!("✅ Datei erfolgreich geladen");
            file
        }
        Err(e) => {
            println!("❌ Fehler beim Laden der Datei: {}", e);
            return Err(e.into());
        }
    };
    
    let mut docx = match docx_file.parse() {
        Ok(doc) => {
            println!("✅ Datei erfolgreich geparst");
            doc
        }
        Err(e) => {
            println!("❌ Fehler beim Parsen der Datei: {}", e);
            return Err(e.into());
        }
    };
    
    // Analyse der Datei
    println!("\n🔍 ANALYSE DER DATEI:");
    println!("═══════════════════════");
    
    let text = docx.document.body.text();
    let paragraph_count = count_paragraphs(&docx);
    let hyperlink_count = count_hyperlinks(&docx);
    
    println!("📊 Statistiken:");
    println!("   📄 Paragraphs: {}", paragraph_count);
    println!("   🔗 Hyperlinks: {}", hyperlink_count);
    println!("   📝 Text-Länge: {} Zeichen", text.len());
    
    // Zeige ersten Teil des Textes
    let preview = if text.len() > 200 {
        format!("{}...", &text[..200])
    } else {
        text.clone()
    };
    println!("   👀 Vorschau: \"{}\"", preview);
    
    // Detaillierte Hyperlink-Analyse
    if hyperlink_count > 0 {
        println!("\n🔗 HYPERLINK-DETAILS:");
        analyze_hyperlinks_detailed(&docx);
    }
    
    // Text-Ersetzungen nur wenn aktiviert
    let mut replacement_count = 0;
    
    if enable_text_replacement {
        println!("\n🔄 FÜHRE TEXT-ERSETZUNGEN DURCH:");
        println!("═══════════════════════════════════");
        
        let replacements = [
            ("Text", "CONTENT"),
            ("Kapitel", "Chapter"),
            ("Überschrift", "Heading"),
            ("Inhaltsverzeichnis", "Table of Contents"),
            ("Seite", "Page"),
            ("1", "ONE"),
            ("2", "TWO"),
            ("hier", "here"),
            ("ist", "is"),
            ("Textmarke", "Bookmark"),
        ];
        
        for (old, new) in &replacements {
            if text.contains(old) {
                println!("   🔄 Ersetze '{}' → '{}'", old, new);
                docx.document.body.replace_text_simple(old, new);
                replacement_count += 1;
            }
        }
        
        if replacement_count == 0 {
            println!("   ℹ️  Keine passenden Wörter für Ersetzung gefunden");
            println!("   🔄 Teste allgemeine Ersetzung...");
            docx.document.body.replace_text_simple("a", "a"); // Sichere Ersetzung
        } else {
            println!("   ✅ {} Ersetzungen durchgeführt", replacement_count);
        }
    } else {
        println!("\n🔄 TEXT-ERSETZUNG:");
        println!("═══════════════════");
        println!("   ⚪ Text-Ersetzung ist deaktiviert (Standard)");
        println!("   💡 Zum Aktivieren: --replace Parameter verwenden");
        println!("   📖 Beispiel: cargo run --example test_docx test.docx --replace");
    }
    
    // Speichern
    println!("\n💾 SPEICHERE VERARBEITETE DATEI:");
    println!("═══════════════════════════════");
    println!("   📁 Speichere als: {}", output_file);
    
    match docx.write_file(&output_file) {
        Ok(_) => {
            println!("   ✅ Datei erfolgreich gespeichert");
        }
        Err(e) => {
            println!("   ❌ Fehler beim Speichern: {}", e);
            return Err(e.into());
        }
    }
    
    // Verifikation - Kann die gespeicherte Datei wieder gelesen werden?
    println!("\n🔍 VERIFIKATION:");
    println!("═══════════════");
    
    match DocxFile::from_file(&output_file) {
        Ok(verification_file) => {
            match verification_file.parse() {
                Ok(verification_docx) => {
                    let final_text = verification_docx.document.body.text();
                    println!("   ✅ Verarbeitete Datei kann wieder gelesen werden");
                    println!("   📝 Finale Text-Länge: {} Zeichen", final_text.len());
                    
                    // Prüfe ob Ersetzungen funktioniert haben
                    if enable_text_replacement {
                        if replacement_count > 0 {
                            let changes_detected = final_text.len() != text.len() || final_text != text;
                            if changes_detected {
                                println!("   ✅ Text-Ersetzungen wurden erfolgreich angewendet");
                            } else {
                                println!("   ⚠️  Keine Text-Änderungen erkannt");
                            }
                        }
                    } else {
                        // Ohne Text-Ersetzung sollte der Text identisch sein
                        if final_text == text {
                            println!("   ✅ Dokument unverändert verarbeitet (wie erwartet)");
                        } else {
                            println!("   ⚠️  Unerwartete Text-Änderungen erkannt");
                        }
                    }
                }
                Err(e) => {
                    println!("   ❌ Verarbeitete Datei kann nicht geparst werden: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Verarbeitete Datei kann nicht geladen werden: {}", e);
        }
    }
    
    // Abschluss
    println!("\n🎉 TEST ABGESCHLOSSEN!");
    println!("═══════════════════════");
    println!("   📁 Original: {}", input_file);
    println!("   📁 Verarbeitet: {}", output_file);
    println!("   💡 Öffne die verarbeitete Datei in Microsoft Word zum finalen Test");
    
    if hyperlink_count > 0 {
        println!("   🎯 Diese Datei enthält Hyperlinks - perfekt zum Testen des Fixes!");
    }
    
    if !enable_text_replacement {
        println!("   📝 Dokument wurde UNVERÄNDERT verarbeitet (keine Text-Ersetzungen)");
        println!("   💡 Für Text-Ersetzungs-Tests: --replace Parameter verwenden");
    }
    
    Ok(())
}

fn count_paragraphs(docx: &Docx) -> usize {
    docx.document.body.content.iter()
        .filter(|content| matches!(content, document::BodyContent::Paragraph(_)))
        .count()
}

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

fn analyze_hyperlinks_detailed(docx: &Docx) {
    let mut hyperlink_number = 1;
    
    for (para_idx, content) in docx.document.body.content.iter().enumerate() {
        if let document::BodyContent::Paragraph(paragraph) = content {
            for para_content in &paragraph.content {
                if let document::ParagraphContent::Link(hyperlink) = para_content {
                    let text = hyperlink.text();
                    let run_count = hyperlink.content.len();
                    
                    println!("   🔗 Hyperlink {}: \"{}\"", hyperlink_number, 
                            truncate_text(&text, 40));
                    println!("      📍 Paragraph: {}", para_idx + 1);
                    println!("      🏃 Runs: {}", run_count);
                    
                    if let Some(id) = &hyperlink.id {
                        println!("      🆔 ID: {}", id);
                    }
                    if let Some(anchor) = &hyperlink.anchor {
                        println!("      ⚓ Anchor: {}", anchor);
                    }
                    
                    // Zeige einzelne Runs (wichtig für Multi-Run Hyperlinks!)
                    if run_count > 1 {
                        println!("      🎯 Multi-Run Hyperlink detected! (Das war das Problem)");
                        for (run_idx, content) in hyperlink.content.iter().enumerate() {
                            match content {
                                document::HyperlinkContent::Run(run) => {
                                    let run_text = run.text();
                                    if !run_text.is_empty() {
                                        println!("         Run {}: \"{}\"", run_idx + 1, 
                                                truncate_text(&run_text, 20));
                                    }
                                }
                            }
                        }
                    }
                    
                    hyperlink_number += 1;
                }
            }
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