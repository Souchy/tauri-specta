use serde::{Serialize, Deserialize};
use specta::datatype::DataType;
use sqlx::{Type, Decode, Encode, Sqlite, sqlite::SqliteValueRef, sqlite::SqliteArgumentValue};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SJson<T>(pub T);

impl<T> Deref for SJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<T> DerefMut for SJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T: specta::Type + 'static> specta::Type for SJson<T> {
    fn inline(type_map: &mut specta::TypeCollection, generics: specta::Generics) -> DataType {
        let dt = T::inline(type_map, generics);
        dt
    }
}

// sqlx::Type for Sqlite
impl<T> Type<Sqlite> for SJson<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
    fn compatible(ty: &sqlx::sqlite::SqliteTypeInfo) -> bool {
        <String as Type<Sqlite>>::compatible(ty)
    }
}

// sqlx::Encode for Sqlite
impl<'q, T> Encode<'q, Sqlite> for SJson<T>
where
    T: Serialize + Send + Sync,
{
    fn encode_by_ref(
        &self,
        buf: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s = serde_json::to_string(&self.0)
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)?;
        <String as Encode<Sqlite>>::encode(s, buf)
    }
}

// sqlx::Decode for Sqlite
impl<'r, T> Decode<'r, Sqlite> for SJson<T>
where
    T: for<'de> Deserialize<'de>,
{
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        let t = serde_json::from_str(&s)?;
        Ok(SJson(t))
    }
}
