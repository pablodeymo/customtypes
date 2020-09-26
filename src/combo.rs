use anyhow::Result;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{validate_name_length, Name};

// permite hacer query de id y name sobre tablas de tipo combo (table_...)
#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct TableComboSimpleQuery {
    pub id: i64,
    pub name: Name,
}

impl TableComboSimpleQuery {
    pub fn from_id_name(id: Option<i64>, name: Option<Name>) -> Option<TableComboSimpleQuery> {
        match (id, name) {
            (Some(id), Some(name)) => Some(TableComboSimpleQuery { id, name }),
            (_, _) => None,
        }
    }
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

// Es un combo simple con parent_id
#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct TableComboWithParentId {
    pub id: i64,
    pub name: Name,
    pub parent_id: i64,
}

// Traits
pub trait SimpleComboTrait {
    fn get_all(conn: &diesel::pg::PgConnection) -> Result<Vec<TableComboSimpleQuery>>;

    fn get_all_combo(conn: &diesel::pg::PgConnection) -> Result<StructComboSimple> {
        let res = Self::get_all(conn)?;
        Ok(StructComboSimple { elements: res })
    }
}

#[cfg(test)]
mod tests {
    use super::TableComboSimpleQuery;
    use crate::Name;
    #[test]
    fn combo_from_id_name() {
        let combo = TableComboSimpleQuery::from_id_name(Some(42), Some(Name::try_from_str("test").unwrap())).unwrap();
        assert_eq!(combo.id, 42);

        let none_combo = TableComboSimpleQuery::from_id_name(Some(42), None);
        assert_eq!(none_combo.is_none(), true);
    }
}