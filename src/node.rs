use {
    uuid::Uuid,
};

pub struct Node {
    pub id: Uuid,
}

impl Node {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }
}