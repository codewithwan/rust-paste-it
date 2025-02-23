use sea_orm::entity::prelude::*;
use ulid::Ulid;
use base62;

/// Entity model for the `paste` table.
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "paste")]
pub struct Model {
    /// The primary key ID of the paste (ULID).
    #[sea_orm(primary_key)]
    pub id: String,
    /// The content of the paste.
    pub content: String,
    /// The short link of the paste.
    pub short_link: String,
    /// The creation date of the paste.
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Generates a short link from a given ULID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ULID to generate the short link from.
    ///
    /// # Returns
    ///
    /// A Base62 encoded short link.
    pub fn generate_shortlink(id: &str) -> String {
        let ulid = Ulid::from_string(id).unwrap();
        base62::encode(ulid.0)
    }
}
