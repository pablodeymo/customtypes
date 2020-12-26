use diesel::deserialize::{self, FromSql};
use diesel::serialize::Output;
use diesel::serialize::ToSql;
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::io::Write;
use strum_macros::EnumString;

#[derive(
    PartialEq, Debug, Copy, Clone, AsExpression, FromSqlRow, EnumString, Deserialize, Serialize,
)]
#[sql_type = "Text"]
pub enum LangEnum {
    #[strum(serialize = "es")]
    ES,
    #[strum(serialize = "en")]
    EN,
    #[strum(serialize = "pt")]
    PT,
}

impl<Pg: diesel::backend::Backend> ToSql<Text, Pg> for LangEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<'_, W, Pg>) -> diesel::serialize::Result {
        let t = match *self {
            LangEnum::ES => "es",
            LangEnum::EN => "en",
            LangEnum::PT => "pt",
        };
        <&str as ToSql<Text, Pg>>::to_sql(&t, out)
    }
}

impl<Pg: diesel::backend::Backend, ST> FromSql<ST, Pg> for LangEnum
where
    String: FromSql<ST, Pg>,
{
    fn from_sql(
        bytes: Option<&<Pg as diesel::backend::Backend>::RawValue>,
    ) -> deserialize::Result<Self> {
        let m = String::from_sql(bytes)?;
        match &m[..] {
            "es" => Ok(LangEnum::ES),
            "en" => Ok(LangEnum::EN),
            "pt" => Ok(LangEnum::PT),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
