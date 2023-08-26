use crate::name::{validate_name_length, Name};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use validator::Validate;

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

    pub fn search_id_from_name(list: &[TableComboSimpleQuery], name: &Name) -> Option<i64> {
        list.iter()
            .filter(|elem| elem.name.as_str() == name.as_str())
            .map(|elem| elem.id)
            .next()
    }

    pub fn search_id_from_name_str(list: &[TableComboSimpleQuery], name_str: &str) -> Option<i64> {
        list.iter()
            .filter(|elem| elem.name.as_str() == name_str)
            .map(|elem| elem.id)
            .next()
    }

    pub fn search_id_from_name_str_max_len(
        list: &[TableComboSimpleQuery],
        name_str: &str,
        max_len: usize,
    ) -> Option<i64> {
        list.iter()
            .filter(|elem| {
                let elem_name_str = elem.name.as_str();
                if elem_name_str.len() > max_len {
                    elem.name.as_str().get(0..max_len) == Some(name_str)
                } else {
                    elem.name.as_str() == name_str
                }
            })
            .map(|elem| elem.id)
            .next()
    }
}

// permite hacer un wrapping de la lista de resultados del combo, para que sea un
// JSON completo
#[derive(Debug, Deserialize, Serialize)]
pub struct StructComboSimple {
    pub elements: Vec<TableComboSimpleQuery>,
}

#[cfg(feature = "enableactix")]
impl From<StructComboSimple> for actix_web::HttpResponse {
    fn from(v: StructComboSimple) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&v).unwrap())
    }
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

impl TableComboWithParentId {
    pub fn search_id_from_name_str(list: &[TableComboWithParentId], name_str: &str) -> Option<i64> {
        list.iter()
            .filter(|elem| elem.name.as_str() == name_str)
            .map(|elem| elem.id)
            .next()
    }
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
    use crate::name::Name;

    #[test]
    fn combo_from_id_name() {
        let combo = TableComboSimpleQuery::from_id_name(
            Some(42),
            Some(Name::try_from_str("test").unwrap()),
        )
        .unwrap();
        assert_eq!(combo.id, 42);

        let none_combo = TableComboSimpleQuery::from_id_name(Some(42), None);
        assert_eq!(none_combo.is_none(), true);
    }

    #[test]
    fn test_search() {
        let elem1 = TableComboSimpleQuery::from_id_name(
            Some(3),
            Some(Name::try_from_str("elem1").unwrap()),
        )
        .unwrap();
        let elem2 = TableComboSimpleQuery::from_id_name(
            Some(5),
            Some(Name::try_from_str("elem5").unwrap()),
        )
        .unwrap();
        let elem3 = TableComboSimpleQuery::from_id_name(
            Some(7),
            Some(Name::try_from_str("elem7").unwrap()),
        )
        .unwrap();
        // new element used to test max len
        let elemnuevo = TableComboSimpleQuery::from_id_name(
            Some(9),
            Some(Name::try_from_str("nuevoelemento").unwrap()),
        )
        .unwrap();

        let v = vec![elem1, elem2, elem3, elemnuevo];
        let id_found =
            TableComboSimpleQuery::search_id_from_name(&v, &Name::try_from_str("elem5").unwrap());
        assert_eq!(id_found.unwrap(), 5);

        let id_not_found =
            TableComboSimpleQuery::search_id_from_name(&v, &Name::try_from_str("zzz").unwrap());
        assert!(id_not_found.is_none());

        let id_found2 = TableComboSimpleQuery::search_id_from_name_str(&v, "elem7");
        assert_eq!(id_found2.unwrap(), 7);

        let id_found_max_len =
            TableComboSimpleQuery::search_id_from_name_str_max_len(&v, "nuevo", 5);
        assert_eq!(id_found_max_len.unwrap(), 9);

        let id_found_max_len_full =
            TableComboSimpleQuery::search_id_from_name_str_max_len(&v, "nuevoelemento", 25);
        assert_eq!(id_found_max_len_full.unwrap(), 9);
    }
}

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct TableComboSimpleReq {
    pub id: Option<i64>,
    pub name: Option<Name>,
}

impl From<TableComboSimpleQuery> for TableComboSimpleReq {
    fn from(v: TableComboSimpleQuery) -> TableComboSimpleReq {
        TableComboSimpleReq {
            id: Some(v.id),
            name: Some(v.name),
        }
    }
}
