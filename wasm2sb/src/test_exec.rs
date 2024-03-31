use eyre::{Context as _, Result};
use wain_ast::ValType;
use wain_exec::{ImportInvalidError, ImportInvokeError, Importer, Memory, Stack};

struct CustomImporter;

impl Importer for CustomImporter {
    const MODULE_NAME: &'static str = "__wasm_sb_bindgen_placeholder__";

    fn validate(
        &self,
        _name: &str,
        _: &[ValType],
        _: Option<ValType>,
    ) -> Option<ImportInvalidError> {
        None
    }

    fn call(
        &mut self,
        name: &str,
        stack: &mut Stack,
        _: &mut Memory,
    ) -> Result<(), ImportInvokeError> {
        if *name == String::from("__wasm_sb_bindgen_u64_split") {
            let val = stack.pop::<i64>();
            println!("call __wasm_sb_bindgen_u64_split: {}", val);
            stack.push((val & 0xffff_ffff) as i32);
            stack.push((val >> 32) as i32);
            println!(
                "call __wasm_sb_bindgen_u64_split: {} {}",
                val & 0xffff_ffff,
                val >> 32
            );
        }

        Ok(())
    }
}

pub fn test_exec() -> Result<()> {
    let buff = std::fs::read("./debug.wasm").wrap_err("failed to read file")?;
    let module = match wain_syntax_binary::parse(buff.as_slice()) {
        Ok(m) => m,
        Err(err) => {
            return Err(eyre::eyre!("{:?}", err.to_string()))
                .wrap_err(format!("failed to parse wasm binary"));
        }
    }
    .module;

    let importer = CustomImporter;

    let mut runtime = wain_exec::Runtime::instantiate(&module, importer)
        .map_err(|e| eyre::eyre!("{:?}", e.to_string()))?;
    match runtime.invoke("nya_sama", &[wain_exec::Value::I32(1000200)]) {
        Ok(k) => {
            if let Some(k) = k {
                match k {
                    wain_exec::Value::I32(i) => println!("return: {}", i),
                    wain_exec::Value::I64(i) => println!("return: {}", i),
                    wain_exec::Value::F32(f) => println!("return: {}", f),
                    wain_exec::Value::F64(f) => println!("return: {}", f),
                }
            } else {
                println!("return: None");
            }
        }
        Err(e) => return Err(eyre::eyre!("{:?}", e.to_string())),
    }

    println!(
        "memory: {:?}",
        runtime.memory().data()[1000200..1000204].to_vec()
    );
    println!(
        "memory: {:?}",
        runtime.memory().data()[1000204..1000208].to_vec()
    );
    println!(
        "memory: {:?}",
        runtime.memory().data()[1000208..1000212].to_vec()
    );
    println!(
        "memory: {:?}",
        runtime.memory().data()[1000212..1000216].to_vec()
    );
    println!("global: {:?}", runtime.module().globals[0]);
    let ret_num = runtime.memory().data()[1000200..1000216].to_vec();
    println!("ret_num: {:?}", ret_num);
    println!(
        "num: {}",
        f64::from_le_bytes([
            ret_num[0], ret_num[1], ret_num[2], ret_num[3], ret_num[4], ret_num[5], ret_num[6],
            ret_num[7],
        ])
    );
    println!(
        "num: {}",
        f64::from_le_bytes([
            ret_num[8],
            ret_num[9],
            ret_num[10],
            ret_num[11],
            ret_num[12],
            ret_num[13],
            ret_num[14],
            ret_num[15],
        ])
    );

    // memoryの0でない場所を探して表示
    let mut index = 0;
    let mut count = 0;
    while index < runtime.memory().data().len() {
        if runtime.memory().data()[index] != 0 {
            let mut str = Vec::new();
            let first = index;
            loop {
                if index >= runtime.memory().data().len() {
                    break;
                }
                if runtime.memory().data()[index] == 0 {
                    break;
                }
                str.push(runtime.memory().data()[index]);
                index += 1;
                count += 1;
            }

            // if index % 4 != 0 {
            //     index += 4 - (index % 4);
            // }

            match std::str::from_utf8(&str) {
                Ok(s) => println!("{}..{}: {}", first, index, s),
                Err(_) => {
                    println!("{}..{}: {:?}", first, index, str)
                }
            }

            // if first % 4 != 0 {
            //     first += 4 - (first % 4);
            // }

            // for i in (first..index - 1).step_by(4) {
            //     let num = i32::from_be_bytes([
            //         runtime.memory().data()[i],
            //         runtime.memory().data()[i + 1],
            //         runtime.memory().data()[i + 2],
            //         runtime.memory().data()[i + 3],
            //     ]);
            //     println!("num: {}..{}: {}", i, i + 4, num);
            // }
        } else {
            index += 1;
        }
    }

    println!("count: {}", count);

    Ok(())
}
