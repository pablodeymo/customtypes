use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{validate_name_length, Name};

// permite hacer query de id y name sobre tablas de tipo combo (table_...)
#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct TableComboSimpleQuery {
    pub id: i64,
    pub name: Name,
}

// permite hacer un wrapping de la lista de resultados del combo, para que sea un
// JSON completo
#[derive(Debug, Deserialize, Serialize)]
pub struct StructComboSimple {
    pub elements: Vec<TableComboSimpleQuery>,
}

// permite recibir el request de insercion de un elemento de
// un combo (tabla)
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ComboInsert {
    #[validate(custom = "validate_name_length")]
    pub name: Name,
}
