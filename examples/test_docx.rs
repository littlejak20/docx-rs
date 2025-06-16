use docx_rust::*;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Cursor;
use zip::{ZipArchive, ZipWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 DOCX Hyperlink-Fix Tester");
    println!("=============================");
    
    // Argumente von Kommandozeile lesen
    let args: Vec<String> = env::args().collect();
    
    // Prüfe auf --help
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return Ok(());
    }
    
    if args.len() < 2 {
        println!("❌ Fehler: Kein Dateiname angegeben\n");
        print_help();
        return Ok(());
    }
    
    // Parse Parameter
    let input_file = &args[1];
    let enable_text_replacement = args.contains(&"--replace".to_string());
    let keep_original_files = args.contains(&"--keep".to_string());
    let fast_mode = args.contains(&"--fast".to_string());
    
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
    
    // Analyse der Datei (überspringen im Fast-Mode)
    let (text, _paragraph_count, hyperlink_count) = if fast_mode {
        println!("\n⚡ FAST-MODE: Analyse übersprungen");
        (String::new(), 0, 0)
    } else {
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
        
        (text, paragraph_count, hyperlink_count)
    };
    
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
        
        if fast_mode {
            // Fast-Mode: Alle Ersetzungen ohne Text-Check
            for (old, new) in &replacements {
                docx.document.body.replace_text_simple(old, new);
                replacement_count += 1;
            }
        } else {
            // Standard: Nur ersetzen wenn Text gefunden
            for (old, new) in &replacements {
                if text.contains(old) {
                    println!("   🔄 Ersetze '{}' → '{}'", old, new);
                    docx.document.body.replace_text_simple(old, new);
                    replacement_count += 1;
                }
            }
        }
        
        if replacement_count == 0 && !fast_mode {
            println!("   ℹ️  Keine passenden Wörter für Ersetzung gefunden");
            println!("   🔄 Teste allgemeine Ersetzung...");
            docx.document.body.replace_text_simple("a", "a"); // Sichere Ersetzung
        } else if !fast_mode {
            println!("   ✅ {} Ersetzungen durchgeführt", replacement_count);
        }
    } else {
        println!("\n🔄 TEXT-ERSETZUNG:");
        println!("═══════════════════");
        println!("   ⚪ Text-Ersetzung ist deaktiviert (Standard)");
        println!("   💡 Zum Aktivieren: --replace Parameter verwenden");
    }
    
    // Speichern - Weiche zwischen den Modi
    println!("\n💾 SPEICHERE VERARBEITETE DATEI:");
    println!("═══════════════════════════════");
    println!("   📁 Speichere als: {}", output_file);
    
    if keep_original_files {
        println!("   🗂️  Modus: Original-Dateien beibehalten (--keep aktiviert)");
        match write_docx_preserving_original_files(&docx, input_file, &output_file) {
            Ok(_) => {
                println!("   ✅ Datei erfolgreich gespeichert (alle Original-Dateien erhalten)");
            }
            Err(e) => {
                println!("   ❌ Fehler beim Speichern: {}", e);
                return Err(e.into());
            }
        }
    } else {
        println!("   📄 Modus: Standard (nur verarbeitete DOCX)");
        match docx.write_file(&output_file) {
            Ok(_) => {
                println!("   ✅ Datei erfolgreich gespeichert");
            }
            Err(e) => {
                println!("   ❌ Fehler beim Speichern: {}", e);
                return Err(e.into());
            }
        }
    }
    
    // Verifikation - Kann die gespeicherte Datei wieder gelesen werden? (überspringen im Fast-Mode)
    if !fast_mode {
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
    } else {
        println!("\n⚡ FAST-MODE: Verifikation übersprungen");
    }
    
    // Abschluss
    println!("\n🎉 TEST ABGESCHLOSSEN!");
    println!("═══════════════════════");
    println!("   📁 Original: {}", input_file);
    println!("   📁 Verarbeitet: {}", output_file);
    println!("   💡 Öffne die verarbeitete Datei in Microsoft Word zum finalen Test");
    
    if hyperlink_count > 0 && !fast_mode {
        println!("   🎯 Diese Datei enthält Hyperlinks - perfekt zum Testen des Fixes!");
    }
    
    if !enable_text_replacement {
        println!("   📝 Dokument wurde UNVERÄNDERT verarbeitet (keine Text-Ersetzungen)");
    }
    
    if keep_original_files {
        println!("   🗂️  Original-Dateien (Bilder, Styles, etc.) wurden beibehalten");
    } else {
        println!("   📄 Nur verarbeitete DOCX ohne Original-Assets");
    }
    
    Ok(())
}


fn print_help() {
    println!("USAGE:");
    println!("    test_docx [OPTIONS] <INPUT_FILE>");
    println!();
    println!("ARGS:");
    println!("    <INPUT_FILE>    The DOCX file to process");
    println!();
    println!("OPTIONS:");
    println!("    --replace       Enable text replacements for testing");
    println!("    --keep          Preserve all original files (images, styles, etc.)");
    println!("    --fast          Skip analysis and verification for faster processing");
    println!("    --help, -h      Print help information");
    println!();
    println!("EXAMPLES:");
    println!("    cargo run --example test_docx document.docx");
    println!("    cargo run --example test_docx document.docx --replace");
    println!("    cargo run --example test_docx document.docx --keep");
    println!("    cargo run --example test_docx document.docx --fast");
    println!("    cargo run --example test_docx document.docx --replace --keep --fast");
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

fn write_docx_preserving_original_files(
    docx: &Docx, 
    original_file: &str, 
    output_file: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Ermittle Original-Dateigröße für Capacity-Optimierung
    let orig_size = std::fs::metadata(original_file)?.len() as usize;
    
    // Klone das DOCX-Objekt und schreibe es in einen temporären Buffer
    let mut docx_clone = docx.clone();
    let docx_reassembled_writer = Cursor::new(Vec::with_capacity(orig_size));
    let finished_writer = docx_clone.write(docx_reassembled_writer)?;
    
    // Öffne die verarbeitete DOCX als ZIP-Archiv
    let mut reassembled_zip_archive = ZipArchive::new(finished_writer)?;
    
    // Öffne die Original-DOCX als ZIP-Archiv
    let original_zip_file = File::open(original_file)?;
    let mut original_zip = ZipArchive::new(original_zip_file)?;
    let original_file_names = original_zip.file_names().map(String::from).collect::<Vec<_>>();
    
    // Erstelle die neue DOCX-Datei als ZIP-Archiv mit optimierter Capacity
    let output_zip_file = File::create(output_file)?;
    let mut new_zip = ZipWriter::new(output_zip_file);
    
    // Kopiere alle Dateien: Original-Dateien außer word/document.xml, diese aus der verarbeiteten Version
    for file_name in original_file_names.into_iter() {
        let zip_file = if file_name == "word/document.xml" {
            // Nimm die verarbeitete word/document.xml (mit DrawingML-Fixes)
            reassembled_zip_archive.by_name("word/document.xml")?
        } else {
            // Nimm alle anderen Dateien aus der Original-DOCX
            original_zip.by_name(&file_name)?
        };
        
        // Kopiere die Datei direkt (behält Kompression und Metadaten bei)
        new_zip.raw_copy_file(zip_file)?;
    }
    
    // Schließe das ZIP-Archiv
    new_zip.finish()?;
    
    Ok(())
}