use serde::{Deserialize, Serialize};

/// Request structure for creating a new paste.
#[derive(Deserialize)]
pub struct CreatePasteRequest {
    pub content: String,
}

/// Response structure for creating a new paste.
#[derive(Serialize)]
pub struct CreatePasteResponse {
    pub status: String,
    pub message: String,
    pub data: CreateDataPaste,
}

/// Data structure for the created paste.
#[derive(Serialize)]
pub struct CreateDataPaste {
    pub short_link: String,
}

/// Response structure for retrieving a paste.
#[derive(Serialize)]
pub struct PasteResponse {
    pub status: String,
    pub message: String,
    pub data: PasteData,
}

/// Data structure for the retrieved paste.
#[derive(Serialize)]
pub struct PasteData {
    pub id: String,
    pub short_link: String,
    pub content: String,
    pub created_at: String,
}
