use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::{info, error};
use crate::entities::paste::{self, Entity as Paste}; 
use crate::models::model::{CreatePasteRequest, CreatePasteResponse, CreateDataPaste, PasteResponse, PasteData}; 
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use ulid::Ulid;
use crate::AppState;

/// Handler for creating a new paste.
///
/// # Arguments
///
/// * `state` - The application state.
/// * `payload` - The request payload containing the paste content.
///
/// # Returns
///
/// A response indicating the result of the paste creation.
pub async fn create_paste(
    State(state): State<AppState>,
    Json(payload): Json<CreatePasteRequest>,
) -> impl IntoResponse {
    let id = Ulid::new().to_string();
    let short_link = paste::Model::generate_shortlink(&id);

    let paste = paste::ActiveModel {
        id: Set(id.clone()),
        content: Set(payload.content),
        short_link: Set(short_link.clone()),
        ..Default::default()
    };

    match paste.insert(&state.db).await {
        Ok(inserted) => {
            info!("Paste created with short_link: {}", inserted.short_link);
            (
                StatusCode::CREATED,
                Json(CreatePasteResponse {
                    status: "success".to_string(),
                    message: "Paste created".to_string(),
                    data: CreateDataPaste {
                        short_link: inserted.short_link,
                    },
                }),
            )
                .into_response()
        }
        Err(err) => {
            error!("Failed to create paste: {}", err);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("Failed to create paste: {}", err),
                })),
            )
                .into_response()
        }
    }
}

/// Handler for retrieving a paste by its short link.
///
/// # Arguments
///
/// * `state` - The application state.
/// * `shortlink` - The short link of the paste to retrieve.
///
/// # Returns
///
/// A response containing the paste data if found, otherwise an error response.
pub async fn get_paste_by_shortlink(
    State(state): State<AppState>,
    Path(shortlink): Path<String>,
) -> impl IntoResponse {
    let shortlink_clone = shortlink.clone();
    match Paste::find()
        .filter(paste::Column::ShortLink.eq(shortlink_clone))
        .one(&state.db)
        .await
    {
        Ok(Some(paste)) => {
            info!("Paste retrieved with short_link: {}", paste.short_link);
            (
                StatusCode::OK,
                Json(PasteResponse {
                    status: "success".to_string(),
                    message: "Paste retrieved".to_string(),
                    data: PasteData {
                        id: paste.id,
                        short_link: paste.short_link,
                        content: paste.content,
                        created_at: paste.created_at.to_rfc3339(),
                    },
                }),
            )
                .into_response()
        }
        Ok(None) => {
            info!("Paste not found with short_link: {}", shortlink.clone());
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "Paste not found",
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("Failed to retrieve paste: {}", err);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("Failed to retrieve paste: {}", err),
                })),
            )
                .into_response()
        }
    }
}
