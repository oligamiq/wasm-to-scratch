pub mod function_code;
pub mod reformat;
pub use reformat::*;
pub mod buddy_block;
pub mod to_utf8;

pub mod block_generator_into {
    use sb_itchy::prelude::*;
    use sb_sbity::value::{Number, Value};
    pub type Bfb = BlockFieldBuilder;
    pub type Bib = BlockInputBuilder;
    pub type Biv = BlockInputValue;

    pub trait BlockGeneratorInto<T> {
        fn to(self) -> T;
    }

    impl BlockGeneratorInto<Bib> for i32 {
        fn to(self) -> Bib {
            Bib::value(Biv::Number {
                value: Value::Number(Number::Int(self as i64)),
            })
        }
    }

    impl BlockGeneratorInto<Bib> for usize {
        fn to(self) -> Bib {
            Bib::value(Biv::Number {
                value: Value::Number(Number::Int(self as i64)),
            })
        }
    }

    impl BlockGeneratorInto<Bib> for &str {
        fn to(self) -> Bib {
            Bib::value(Biv::String {
                value: Value::Text(self.into()),
            })
        }
    }
}
