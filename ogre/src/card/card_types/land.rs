use uuid::Uuid;

#[derive(Clone)]
pub struct Land {
    id: Uuid,
    owner_id: Uuid,
    name: String,

    // Land specific fields
    is_tapped: bool,
}

impl Land {
    pub fn new(owner_id: Uuid, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner_id,
            name,
            is_tapped: false,
        }
    }
}
