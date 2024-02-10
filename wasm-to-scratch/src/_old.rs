use wasmparser::{Chunk, FunctionBody, Parser, Payload::*};
use std::io::Read;
use anyhow::Result;

fn main() {
    let filepath = "./testcode/testcode.wasm";
    let wasm = std::fs::File::open(filepath).unwrap();
    parse(wasm).unwrap();
}

// https://docs.rs/wasmparser/latest/wasmparser/struct.Parser.html#method.new
fn parse(mut reader: impl Read) -> Result<()> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let parser = Parser::new(0);

    for payload in parser.parse_all(&buf) {
        match payload? {
            // Sections for WebAssembly modules
            Version { num, encoding, range } => {
                println!("Version: {}", num);
                println!("Encoding: {:?}", encoding);
            }
            TypeSection(import_payload) => {
                println!("TypeSectionCount: {}", import_payload.count());
            }
            ImportSection(import_section) => {
                println!("ImportSectionCount: {}", import_section.count());
            }
            FunctionSection(function_section) => {
                println!("FunctionSectionCount: {}", function_section.count());
            }
            TableSection(table_section) => {
                println!("TableSectionCount: {}", table_section.count());
            }
            MemorySection(memory_section) => {
                println!("MemorySectionCount: {}", memory_section.count());
            }
            TagSection(tag_section) => {
                println!("TagSectionCount: {}", tag_section.count());
            }
            GlobalSection(global_section) => {
                println!("GlobalSectionCount: {}", global_section.count());
            }
            ExportSection(export_section) => {
                println!("ExportSectionCount: {}", export_section.count());
            }
            StartSection { func, range } => {
                println!("StartSection: {:?}", func);
            }
            ElementSection(element_section) => {
                println!("ElementSectionCount: {}", element_section.count());
            }
            DataCountSection { count, range } => {
                println!("DataCountSection: {}", count);
            }
            DataSection(data_section) => {
                println!("DataSectionCount: {}", data_section.count());
            }

            // Here we know how many functions we'll be receiving as
            // `CodeSectionEntry`, so we can prepare for that, and
            // afterwards we can parse and handle each function
            // individually.
            CodeSectionStart { count, range, size } => {
                println!("CodeSectionStart: {}", count);
                println!("CodeSectionStart: {:?}", range);
            }
            CodeSectionEntry(body) => {
                let reader = body.get_binary_reader();
                println!("CodeSectionEntrySize: {:?}", reader.bytes_remaining());
                println!("CodeSectionEntry: {:?}", reader.range());
                // println!("CodeSectionEntry: {:?}", body.get_operators_reader());
                println!("CodeSectionEntry: {:?}", reader);
            }

            // Sections for WebAssembly components
            ModuleSection { parser, range } => {
                println!("ModuleSection: {:?}", range);
            }
            InstanceSection(instance_section) => {
                println!("InstanceSectionCount: {}", instance_section.count());
            }
            CoreTypeSection(core_type_section) => {
                println!("CoreTypeSectionCount: {}", core_type_section.count());
            }
            ComponentSection { parser, range } => {
                println!("ComponentSection: {:?}", range);
            }
            ComponentInstanceSection(component_instance_section) => {
                println!("ComponentInstanceSectionCount: {}", component_instance_section.count());
            }
            ComponentAliasSection(component_alias_section) => {
                println!("ComponentAliasSectionCount: {}", component_alias_section.count());
            }
            ComponentTypeSection(component_type_section) => {
                println!("ComponentTypeSectionCount: {}", component_type_section.count());
            }
            ComponentCanonicalSection(component_canonical_section) => {
                println!("ComponentCanonicalSectionCount: {}", component_canonical_section.count());
            }
            ComponentStartSection { start, range } => {
                println!("ComponentStartSection: {:?}", start);
            }
            ComponentImportSection(component_import_section) => {
                println!("ComponentImportSectionCount: {}", component_import_section.count());
            }
            ComponentExportSection(component_export_section) => {
                println!("ComponentExportSectionCount: {}", component_export_section.count());
            }
            CustomSection(custom_section) => {
                println!("CustomSection: {:?}", custom_section);
            }

            // most likely you'd return an error here
            UnknownSection { id, .. } => { /* ... */ }

            // Once we've reached the end of a parser we either resume
            // at the parent parser or the payload iterator is at its
            // end and we're done.
            End(_) => {}

            _ => {}
        }
    }

    Ok(())
}
