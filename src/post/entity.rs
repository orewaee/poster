use sqlx::{FromRow, Sqlite, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash, sqlx::Encode, sqlx::Decode)]
pub struct PostId(String);

impl PostId {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Default for PostId {
    fn default() -> Self {
        Self::new(&Uuid::new_v4().to_string())
    }
}

impl std::fmt::Display for PostId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for PostId {
    fn from(value: String) -> Self {
        PostId(value)
    }
}

impl<'a> From<&'a str> for PostId {
    fn from(value: &'a str) -> Self {
        PostId(value.to_string())
    }
}

impl Into<String> for PostId {
    fn into(self) -> String {
        self.0
    }
}

impl Type<Sqlite> for PostId {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <String as Type<Sqlite>>::type_info()
    }

    fn compatible(ty: &<Sqlite as sqlx::Database>::TypeInfo) -> bool {
        <String as Type<Sqlite>>::compatible(ty)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct Post {
    pub id: PostId,
    pub password: Option<String>,
}
