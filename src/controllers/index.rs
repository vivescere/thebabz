use axum::{extract::State, response::Html};
use minijinja::context;

use crate::state::AppState;

/// Homepage of the application.
pub async fn index(
    State(state): State<AppState>,
) -> Html<String> {
    // TODO: app errors
    let template = state.templates.get_template("index.html").unwrap();
    let context = context!(name => "World");
    let source = template.render(context).unwrap();
    Html(source)
}
