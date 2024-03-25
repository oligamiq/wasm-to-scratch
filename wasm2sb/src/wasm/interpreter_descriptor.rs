use std::{process::exit, sync::Arc};

use parking_lot::RwLock;
use wain_ast::{Module, ValType};
use wain_exec::{ImportInvalidError, ImportInvokeError, Importer, Memory, Runtime, Stack};

#[derive(Clone)]
struct CounterImporter {
    count: Arc<RwLock<Vec<u32>>>,
}

impl Importer for CounterImporter {
    const MODULE_NAME: &'static str = "__wasm_sb_bindgen_placeholder__";

    fn validate(
        &self,
        _name: &str,
        _: &[ValType],
        _: Option<ValType>,
    ) -> Option<ImportInvalidError> {
        // println!("validate name: {}", name);
        None
    }

    fn call(
        &mut self,
        name: &str,
        stack: &mut Stack,
        _: &mut Memory,
    ) -> Result<(), ImportInvokeError> {
        // println!("call name: {}", name);
        if *name == String::from("__wasm_sb_bindgen_describe") {
            let mut count = self.count.write();
            count.push(stack.pop::<f64>() as u32);
            Ok(())
        } else if *name == String::from("__wasm_sb_bindgen_debug_num") {
            println!("debug {}", stack.pop::<i32>());
            Ok(())
        } else if *name == String::from("__wasm_sb_bindgen_describe_closure") {
            // fn __wasm_sb_bindgen_describe_closure(a: f64, b: f64, c: f64) -> f64;
            println!(
                "__wasm_sb_bindgen_describe_closure:  {}",
                stack.pop::<f64>()
            );
            println!(
                "__wasm_sb_bindgen_describe_closure:  {}",
                stack.pop::<f64>()
            );
            println!(
                "__wasm_sb_bindgen_describe_closure:  {}",
                stack.pop::<f64>()
            );
            stack.push(3.0);
            Ok(())
        } else if *name == String::from("__wasm_sb_bindgen_object_clone_ref") {
            // fn __wasm_sb_bindgen_object_clone_ref(idx: f64) -> f64;
            let idx = stack.pop::<f64>();
            println!("__wasm_sb_bindgen_object_clone_ref: idx: {}", idx);
            stack.push(idx);
            Ok(())
        } else {
            eprintln!("unknown function: {}", name);
            unreachable!()
        }
    }
}

impl CounterImporter {
    pub fn new() -> Self {
        Self {
            count: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn reset(&mut self) {
        self.count.write().clear();
    }

    pub fn get_count(&self) -> Vec<u32> {
        self.count.read().clone()
    }
}

// https://github.com/rhysd/wain/tree/master/wain-exec
pub fn interpreter_descriptor(module: &Module, fn_names: Vec<String>) -> Vec<(String, Vec<u32>)> {
    let mut importer = CounterImporter::new();

    // println!("interpreter_descriptor");

    // Make abstract machine runtime. It instantiates a module instance
    let mut runtime = match Runtime::instantiate(&module, importer.clone()) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("could not instantiate module: {}", err);
            exit(1);
        }
    };

    // println!("interpreter_descriptor 2");

    let d = fn_names
        .iter()
        .map(|fn_name| {
            // println!("interpreter_descriptor fn_name: {}", fn_name);

            importer.reset();

            // non return
            match runtime.invoke(fn_name, &[]) {
                Ok(ret) => {
                    // `ret` is type of `Option<Value>` where it contains `Some` value when the invoked
                    // function returned a value. Otherwise it's `None` value.
                    match ret {
                        Some(_) => unreachable!(),
                        None => {}
                    }
                }
                Err(trap) => eprintln!("Execution was trapped: {}", trap),
            };

            (fn_name.clone(), importer.get_count())
        })
        .collect::<Vec<_>>();
    d
}
