use serde::{Deserialize, Serialize};
use specta::{DataType, Type};

/// A wrapper type for `sqlx::types::Json<T>` that implements `Type` by forwarding to the inner type `T`.
#[derive(Serialize, Deserialize)] // Type
#[serde(transparent)]
pub struct SqlxJson<T>(pub sqlx::types::Json<T>);
impl SqlxJson<()> {
    pub fn new<T>(value: T) -> SqlxJson<T> {
        SqlxJson(sqlx::types::Json(value))
    }
}

impl<T: Type + 'static> Type for SqlxJson<T> {
    fn inline(type_map: &mut specta::TypeCollection, generics: specta::Generics) -> DataType {
        let dt = T::inline(type_map, generics);
        dt
    }
}
