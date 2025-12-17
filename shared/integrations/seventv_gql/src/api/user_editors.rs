use crate::http_client::{HttpClient, HttpClientAuthed};

#[derive(Clone)]
pub struct UserEditorService<C> {
    client: C,
}

impl<C: HttpClient> UserEditorService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

impl<C: HttpClientAuthed> UserEditorService<C> {
    pub async fn add_editor(&self) {}
}
