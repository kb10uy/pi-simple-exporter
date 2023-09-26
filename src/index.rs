use axum::{http::StatusCode, response::Html};
use yarte::{auto as yarte_auto, ywrite_html};

#[derive(Debug, Clone)]
struct IndexData {
    under_voltage: u64,
}

pub async fn render_index() -> (StatusCode, Html<String>) {
    let data = IndexData { under_voltage: 0 };
    let rendered = yarte_auto!(ywrite_html!(String, "{{> index data }}"));
    (StatusCode::OK, Html(rendered))
}
