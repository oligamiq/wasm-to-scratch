use std::{process::exit, sync::Arc};

use parking_lot::RwLock;
use sb_itchy::stack;
use wain_ast::ValType;
use wain_exec::{ImportInvalidError, ImportInvokeError, Importer, Memory, Runtime, Stack};

#[derive(Clone)]
struct CounterImporter {
    count: Arc<RwLock<Vec<u32>>>,
}

impl Importer for CounterImporter {
    const MODULE_NAME: &'static str = "__wasm_sb_bindgen_placeholder__";

    fn validate(&self, name: &str, _: &[ValType], _: Option<ValType>) -> Option<ImportInvalidError> {
        // println!("validate name: {}", name);
        None
    }

    fn call(&mut self, name: &str, stack: &mut Stack, _: &mut Memory) -> Result<(), ImportInvokeError> {
        // println!("call name: {}", name);
        if *name == String::from("__wasm_sb_bindgen_describe") {
            let mut count = self.count.write();
            count.push(stack.pop::<i32>() as u32);
            Ok(())
        } else {
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
pub fn interpreter_descriptor(module: &wain_ast::Module, fn_names: Vec<String>) -> Vec<(String, Vec<u32>)> {
    let mut importer = CounterImporter::new();

    // Make abstract machine runtime. It instantiates a module instance
    let mut runtime = match Runtime::instantiate(&module, importer.clone()) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("could not instantiate module: {}", err);
            exit(1);
        }
    };

    let d = fn_names
        .iter()
        .map(|fn_name| {
            importer.reset();

            // non return
            match runtime.invoke(fn_name, &[]) {
                Ok(ret) => {
                    // `ret` is type of `Option<Value>` where it contains `Some` value when the invoked
                    // function returned a value. Otherwise it's `None` value.
                    match ret {
                        Some(_) => unreachable!(),
                        None => {},
                    }
                }
                Err(trap) => eprintln!("Execution was trapped: {}", trap),
            };

            (fn_name.clone(), importer.get_count())
        })
        .collect::<Vec<_>>();
    d
}
