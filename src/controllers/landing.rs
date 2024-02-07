use axum::{extract::State, response::Html};
use minijinja::context;

use crate::state::AppState;
use crate::errors::ServiceError;

/// Landing page of the application.
pub async fn landing(
    State(state): State<AppState>,
) -> Result<Html<String>, ServiceError> {
    let template = state.templates.get_template("landing/landing.html.j2")?;
    let context = context!();
    let source = template.render(context)?;
    Ok(Html(source))
}
