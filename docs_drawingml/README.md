# docx-rust Improvements Project Template

This template contains everything necessary for improving the docx-rs crate based on forensic analysis of real DOCX corruptions.

## 🚀 **Quick Start**

1. **Copy this template folder** into your new docx-rust project
2. **Read ../CLAUDE_DRAWINGML.md** - Complete guide for Claude (DrawingML fixes)
3. **Follow TODO_ROADMAP.md** - Prioritized implementation steps
4. **Use reference_materials/** - Ready-made code templates and XML examples

## 📁 **Template Structure**

```
project_template_docx_rust/
├── CLAUDE_DRAWINGML.md                 # ← MAIN GUIDE for new Claude session (DrawingML fixes)
├── README.md                           # ← This overview
├── TODO_ROADMAP.md                     # ← Concrete implementation steps
├── reference_materials/                # ← All important references
│   ├── xml_examples/                   # ← Critical XML comparisons
│   │   ├── broken_drawingml.xml        # ← Broken version (won't open in Word)
│   │   ├── working_drawingml.xml       # ← Working original
│   │   └── fixed_drawingml.xml         # ← Repaired version
│   ├── rust_code_templates/            # ← Ready-made code templates
│   └── analysis_references.md          # ← Links to detailed analyses
└── development_guide/                  # ← Development assistance
    ├── implementation_steps.md         # ← Step-by-step plan
    ├── test_strategy.md                # ← Testing approach with real documents
    └── success_criteria.md             # ← Measurable success goals
```

## 🎯 **Mission**

**Problem:** DOCX files with complex image positioning (wp:anchor) become unopenable in Microsoft Word due to missing Required Elements.

**Solution:** Robust DrawingML implementation in docx-rs with:
- Required Elements Implementation (`positionH.align/posOffset`, `effectExtent`, `graphicFrameLocks`)
- Safe Mode (`wp:anchor` → `wp:inline` fallback) 
- Boolean Serialization fix (`true/false` → `0/1`) as secondary fix
- Complete OOXML schema compliance

**Impact:** 90% of all DrawingML problems solved with 1-2 weeks effort.

## ⚡ **Phase 1 - Critical Fixes (Immediate Start)**

### **1. Required Elements Implementation (Day 1-3)**
- **Problem:** `<positionH relativeFrom="column"/>` → incomplete, Word cannot open  
- **Fix:** `<positionH><align>center</align></positionH>` → complete, Word opens
- **Code:** Complete wp:anchor structs with validation

### **2. Safe Mode (Day 4-6)**
- **Problem:** Complex `wp:anchor` fragile against XML errors
- **Fix:** Automatic conversion to robust `wp:inline`
- **Code:** See `reference_materials/rust_code_templates/safe_mode_structs.rs`

### **3. Boolean Serialization (Day 7)**
- **Problem:** `behindDoc="false"` → inconsistent (secondary problem)
- **Fix:** `behindDoc="0"` → OOXML standard
- **Code:** See `reference_materials/rust_code_templates/boolean_serialization.rs`

### **4. Validation (Day 8-10)**
- **Test:** With real problem documents from `xml_examples/`
- **Goal:** `broken_drawingml.xml` → working DOCX

## 📊 **Forensically Validated**

This template is based on:
- **30+ analyzed DOCX files** (original, anonymized, repaired)
- **Real problem cases** from Legal Tech (court documents)
- **Successful repairs** with validated fixes
- **Word compatibility tests** with Microsoft Word

## 🔗 **For Claude: Complete Context in CLAUDE_DRAWINGML.md**

The `CLAUDE_DRAWINGML.md` contains:
- ✅ **Complete project context** without external dependencies
- ✅ **Inline XML examples** (broken vs. working)
- ✅ **Ready-made Rust code templates** 
- ✅ **3-phase implementation plan** with time estimates
- ✅ **Test strategy** with real problem documents
- ✅ **Measurable success criteria**
- ✅ **OOXML standards** and development guidelines

**Goal:** A new Claude session can work productively immediately, without needing the entire investigation context.

## ⚠️ **Usage**

1. **Copy this template folder** into your docx-rust project directory
2. **Start new Claude session** focusing on `CLAUDE_DRAWINGML.md`
3. **Fork docx-rs repository** and follow implementation steps
4. **Test with real problem documents** from `xml_examples/`

The template is **self-contained** and contains all necessary information for successful implementation of DrawingML fixes.