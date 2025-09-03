use maplit::hashset;

use crate::stub_type::ImportRef;

use super::{PyStubType, TypeInfo};

impl PyStubType for rust_decimal::Decimal {
    fn type_output() -> TypeInfo {
        TypeInfo {
            name: "decimal.Decimal".to_string(),
            import: hashset! { ImportRef::Module("decimal".into()) },
        }
    }
}
