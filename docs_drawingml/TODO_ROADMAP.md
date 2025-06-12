# TODO Roadmap - docx-rust DrawingML Improvements

## 🚀 **Phase 1: Critical Fixes (1-2 Weeks) - HIGHEST PRIORITY**

### **Day 1-3: Setup & Required Elements Implementation**
- [ ] **Repository Setup**
  ```bash
  git clone https://github.com/littlejak20/docx-rs.git
  cd docx-rs
  git checkout -b feature/drawingml-robustness
  ```

- [ ] **Codebase Analysis**
  - [ ] Find DrawingML/wp:anchor implementation
  - [ ] Locate boolean serialization in code
  - [ ] Identify test setup and existing DrawingML tests

- [ ] **CRITICAL: Required Elements Implementation**
  - [ ] Implement complete `PositionH`/`PositionV` structs with validation
  - [ ] Add `EffectExtent`, `CnvGraphicFramePr`, `PicLocks` structs
  - [ ] Validation: `positionH` MUST have `align` XOR `posOffset`
  - [ ] Test: Generated XML must contain complete DrawingML structure

### **Day 4-6: Safe Mode Implementation**
- [ ] **Safe Mode Structs**
  - [ ] Implement `Drawing::sanitize_for_compatibility()`
  - [ ] `anchor_to_simple_inline()` conversion logic
  - [ ] Fallback for missing/invalid DrawingML elements

- [ ] **API Extension**
  - [ ] Add `Docx::save_compatible()` method
  - [ ] `sanitize_all_drawings()` for complete document
  - [ ] Logging/warnings for conversion events

### **Day 7: Boolean Serialization Fix**
- [ ] **Boolean Serialization (SECONDARY)**
  - [ ] Implement `serialize_bool_ooxml()` function
  - [ ] Replace all `true/false` with `0/1` in DrawingML context
  - [ ] Test: Generated XML must contain `behindDoc="0"` instead of `behindDoc="false"`

### **Day 8-10: Testing & Validation**
- [ ] **Unit Tests**
  - [ ] Required Elements validation tests
  - [ ] Safe Mode conversion tests  
  - [ ] Boolean serialization tests
  - [ ] XML output validation

- [ ] **Integration Tests**
  - [ ] Test with `reference_materials/xml_examples/broken_drawingml.xml`
  - [ ] Expectation: After fix, DOCX should be openable in Word
  - [ ] Regression tests: All existing tests must pass

## 🎯 **Phase 1 Success Criteria (MUST be fulfilled)**
- [ ] ✅ **Required Elements:** Complete wp:anchor structs with positionH.align/posOffset
- [ ] ✅ **DrawingML Validation:** effectExtent, graphicFrameLocks etc. implemented
- [ ] ✅ **Safe Mode:** `save_compatible()` converts wp:anchor to wp:inline  
- [ ] ✅ **Real-World Test:** Problem document opens after fix in Microsoft Word
- [ ] ✅ **Boolean Format:** `0/1` instead of `true/false` in all DrawingML elements
- [ ] ✅ **Regression:** All existing tests continue to pass
- [ ] ✅ **Performance:** No deterioration in build/runtime

---

## 🔧 **Phase 2: Complete DrawingML (OPTIONAL - 2-4 Weeks)**

### **Week 1: Extended Structs**
- [ ] **Extend WpAnchor Struct**
  - [ ] Implement all attributes from `working_drawingml.xml`
  - [ ] PositionH/PositionV with complete align support
  - [ ] EffectExtent, GraphicFramePr, SizeRel structs

- [ ] **Required Elements Implementation**
  - [ ] `<wp:effectExtent>` for shadows/effects
  - [ ] `<wp:cNvGraphicFramePr>` with GraphicFrameLocks
  - [ ] `<a:picLocks>` for picture properties
  - [ ] `<wp:sizeRelH>`/`<wp:sizeRelV>` for relative sizes

### **Week 2: XML Serialization**
- [ ] **Complete XML Generation**
  - [ ] Correctly serialize all DrawingML elements
  - [ ] Namespace management (wp: vs wp14:)
  - [ ] OOXML schema validation

### **Week 3-4: Testing & Optimization**
- [ ] **Comprehensive Tests**
  - [ ] Tests with all `xml_examples/`
  - [ ] Performance benchmarks
  - [ ] Memory usage validation

---

## 🚀 **Phase 3: Advanced Features (OPTIONAL - 1-2 Weeks)**

- [ ] **EMF-specific Optimizations**
  - [ ] Better handling for Enhanced Metafile Format
  - [ ] EMF vs PNG/JPEG distinction

- [ ] **Advanced Features**
  - [ ] DrawingML extensions support
  - [ ] Namespace versioning
  - [ ] Performance optimizations for large documents

---

## 🧪 **Continuous Validation Checklist**

### **Check with every change:**
- [ ] **Required Elements Test:** positionH has align XOR posOffset
- [ ] **XML Test:** Generated XML conforms to OOXML schema  
- [ ] **Word Test:** Test DOCX opens in Microsoft Word without errors
- [ ] **Boolean Test:** `grep -r "true\|false" src/` should show no DrawingML booleans
- [ ] **Regression Test:** `cargo test` runs successfully

### **Real-World Test Documents:**
- [ ] `broken_drawingml.xml` → Openable in Word after fix
- [ ] `working_drawingml.xml` → Remains compatible  
- [ ] `fixed_drawingml.xml` → Serves as reference for correct structure

## 📊 **Definition of Done**

### **Phase 1 - Minimal Viable Fix:**
- Required Elements fully implemented (positionH.align/posOffset etc.)
- Safe Mode converts problematic wp:anchor to wp:inline
- `save_compatible()` API available  
- Boolean serialization uses `0/1` format
- Problem document from XML examples opens in Word
- All existing tests pass

### **Phase 2 - Full Feature Support:**
- Complete wp:anchor implementation
- All critical DrawingML elements supported
- XML output identical to Microsoft Word
- Feature parity for complex positioning

### **Phase 3 - Production Ready:**
- EMF format optimally supported
- Performance optimized for large documents
- Comprehensive documentation and examples
- Community feedback incorporated

## ⚡ **Quick Start Commands**

```bash
# Setup
git clone https://github.com/littlejak20/docx-rs.git && cd docx-rs
git checkout -b feature/drawingml-robustness

# Development
cargo build
cargo test
cargo test drawingml  # DrawingML-spezifische Tests

# Validation
cargo run --example basic  # Test mit Beispiel-DOCX
# → Manuell in Word öffnen und prüfen

# Integration
git add . && git commit -m "feat: fix DrawingML boolean serialization"
git push origin feature/drawingml-robustness
```

## 🔗 **References During Development**

- **Code Templates:** `reference_materials/rust_code_templates/`
- **XML Comparisons:** `reference_materials/xml_examples/`
- **Detailed Analysis:** `../drawingml_analysis/DOCX_RS_VERBESSERUNGSVORSCHLAEGE.md`
- **Test Documents:** `../docx_files/` (30+ real DOCX examples)

---

**IMPORTANT:** Phase 1 is **CRITICAL** and solves 90% of all problems. Phase 2+3 are **OPTIONAL** and should only be implemented after complete requirements analysis if needed.

**CORRECTION:** Required Elements Implementation (positionH.align/posOffset, effectExtent, graphicFrameLocks) has **HIGHEST PRIORITY**. Boolean serialization is **secondary** - alone it does not make documents unopenable.