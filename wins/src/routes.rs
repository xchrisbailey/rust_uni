use axum::response::IntoResponse;

use crate::templates::{global_template::HtmlTemplate, index_template::IndexTemplate};

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}
