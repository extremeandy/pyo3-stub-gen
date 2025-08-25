use maplit::hashset;

use super::{PyStubType, TypeInfo};

impl PyStubType for rust_decimal::Decimal {
    fn type_output() -> TypeInfo {
        TypeInfo {
            name: "decimal.Decimal".to_string(),
            import: hashset! { "decimal".into() },
        }
    }
}
