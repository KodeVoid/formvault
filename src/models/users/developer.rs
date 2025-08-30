use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Developer {
    name: String,
    email: String,
    public_key: String,
    id: Uuid,
}

impl Developer {
    // add code here

    fn new(name: String, email: String, public_key: String) -> Self {
        Developer {
            name,
            email,
            public_key,
            id: Uuid::new_v4(),
        }
    }
}
