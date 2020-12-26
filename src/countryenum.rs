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
pub enum CountryEnum {
    #[strum(serialize = "AR")]
    AR,
    #[strum(serialize = "BR")]
    BR,
    #[strum(serialize = "CL")]
    CL,
    #[strum(serialize = "CO")]
    CO,
    #[strum(serialize = "PE")]
    PE,
    #[strum(serialize = "PY")]
    PY,
    #[strum(serialize = "UY")]
    UY,
}

impl<Pg: diesel::backend::Backend> ToSql<Text, Pg> for CountryEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<'_, W, Pg>) -> diesel::serialize::Result {
        let t = match *self {
            CountryEnum::AR => "AR",
            CountryEnum::BR => "BR",
            CountryEnum::CL => "CL",
            CountryEnum::CO => "CO",
            CountryEnum::PE => "PE",
            CountryEnum::PY => "PY",
            CountryEnum::UY => "UY",
        };
        <&str as ToSql<Text, Pg>>::to_sql(&t, out)
    }
}

impl<Pg: diesel::backend::Backend, ST> FromSql<ST, Pg> for CountryEnum
where
    String: FromSql<ST, Pg>,
{
    fn from_sql(
        bytes: Option<&<Pg as diesel::backend::Backend>::RawValue>,
    ) -> deserialize::Result<Self> {
        let m = String::from_sql(bytes)?;
        match &m[..] {
            "AR" => Ok(CountryEnum::AR),
            "BR" => Ok(CountryEnum::BR),
            "CL" => Ok(CountryEnum::CL),
            "CO" => Ok(CountryEnum::CO),
            "PE" => Ok(CountryEnum::PE),
            "PY" => Ok(CountryEnum::PY),
            "UY" => Ok(CountryEnum::UY),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
