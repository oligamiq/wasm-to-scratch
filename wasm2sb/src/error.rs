use cargo_metadata::diagnostic;
use miette::{Diagnostic, SourceSpan};
use parking_lot::Mutex;
use thiserror::Error;

static ERRORS: Mutex<Vec<Box<dyn Diagnostic + 'static + Send + Sync>>> = Mutex::new(Vec::new());

#[derive(Error, Debug, Diagnostic)]
#[error("oops!")]
#[diagnostic(
    url(docsrs),
)]
pub struct Wasm2SbError {
    #[related]
    others: Vec<Box<dyn miette::Diagnostic + 'static + Send + Sync>>,
}

impl Wasm2SbError {
    pub fn return_with_error<E: miette::Diagnostic + 'static + Send + Sync>(err: E) -> miette::Report {
        ERRORS.lock().push(Box::new(err));
        Wasm2SbError {
            others: *ERRORS.lock(),
        }.into()
    }

    pub fn add_warning(&self, err: Box<dyn miette::Diagnostic + 'static + Send + Sync>) -> Self {
        ERRORS.lock().push(err);
        Wasm2SbError {
            others: *ERRORS.lock(),
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("file is not found!")]
#[diagnostic(
    code(file::not::found),
    url(docsrs),
    help("repair the file path"),
    severity(Error)
)]

pub struct FileNotFoundError {
    #[source_code]
    pub src: String,
    #[label("file not found")]
    pub bad_file: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[diagnostic(
    code(wasm::file::broken),
    url(docsrs),
    severity(Error)
)]
pub struct WasmError {
    #[source_code]
    pub src: String,
    #[label("wasm is broken")]
    pub bad_bit: SourceSpan,
    #[help]
    advice: String,
}
