use axum::{response::{IntoResponse, Response, Html}, http::StatusCode};

/// Defines a list of all of the possible ways the application services,
/// meaning its routes, can fail. These errors are *not* meant to be
/// gracefully handled, they are failures (database not up, ...).
pub enum ServiceError {
    Templating(minijinja::Error),
}

impl From<minijinja::Error> for ServiceError {
    fn from(error: minijinja::Error) -> Self {
        Self::Templating(error)
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        match self {
            Self::Templating(error) => tracing::error!(variant = "Templating", %error, "A fatal error occured"),
        }

        // The templating engine is intentionally not used here, since it is fallible.
        // As this is an error handler, we want to avoid nested errors.
        let source = include_str!("../templates/errors/500.html");

        (StatusCode::INTERNAL_SERVER_ERROR, Html(source)).into_response()
    }
}
