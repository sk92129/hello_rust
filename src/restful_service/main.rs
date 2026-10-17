use std::error::Error;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    connection: Arc<Mutex<Connection>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Product {
    id: i64,
    name: String,
    description: String,
    price_cents: i64,
}

#[derive(Deserialize)]
struct NewProduct {
    name: String,
    description: String,
    price_cents: i64,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

struct ApiError {
    status: StatusCode,
    message: &'static str,
}

impl ApiError {
    fn bad_request() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: "name and description must not be empty, and price_cents must be non-negative",
        }
    }

    fn internal(error: impl Display) -> Self {
        eprintln!("Catalog request failed: {error}");
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "internal server error",
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorResponse {
                error: self.message,
            }),
        )
            .into_response()
    }
}

fn initialize_catalog(connection: &Connection) -> rusqlite::Result<()> {
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS products (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            name         TEXT NOT NULL UNIQUE,
            description  TEXT NOT NULL,
            price_cents  INTEGER NOT NULL CHECK (price_cents >= 0)
        );",
    )?;

    for (name, description, price_cents) in [
        ("Coffee", "Whole bean medium roast coffee", 1299),
        ("Tea", "Loose-leaf breakfast tea", 899),
        ("Mug", "Ceramic mug, 350 ml", 1499),
    ] {
        connection.execute(
            "INSERT OR IGNORE INTO products (name, description, price_cents)
             VALUES (?1, ?2, ?3)",
            params![name, description, price_cents],
        )?;
    }

    Ok(())
}

fn app(connection: Connection) -> Router {
    Router::new()
        .route("/api/catalog", get(get_catalog).post(post_catalog))
        .with_state(AppState {
            connection: Arc::new(Mutex::new(connection)),
        })
}

async fn get_catalog(State(state): State<AppState>) -> Result<Json<Vec<Product>>, ApiError> {
    let connection = Arc::clone(&state.connection);
    let products = tokio::task::spawn_blocking(move || {
        let connection = connection.lock().map_err(ApiError::internal)?;
        let mut statement = connection
            .prepare("SELECT id, name, description, price_cents FROM products ORDER BY id")
            .map_err(ApiError::internal)?;
        let products = statement
            .query_map([], |row| {
                Ok(Product {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    price_cents: row.get(3)?,
                })
            })
            .map_err(ApiError::internal)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(ApiError::internal)?;
        Ok::<_, ApiError>(products)
    })
    .await
    .map_err(ApiError::internal)??;

    Ok(Json(products))
}

async fn post_catalog(
    State(state): State<AppState>,
    Json(product): Json<NewProduct>,
) -> Result<(StatusCode, Json<Product>), ApiError> {
    let name = product.name.trim().to_owned();
    let description = product.description.trim().to_owned();
    if name.is_empty() || description.is_empty() || product.price_cents < 0 {
        return Err(ApiError::bad_request());
    }

    let connection = Arc::clone(&state.connection);
    let product = tokio::task::spawn_blocking(move || {
        let connection = connection.lock().map_err(ApiError::internal)?;
        connection
            .execute(
                "INSERT INTO products (name, description, price_cents)
                 VALUES (?1, ?2, ?3)",
                params![name, description, product.price_cents],
            )
            .map_err(ApiError::internal)?;
        Ok::<_, ApiError>(Product {
            id: connection.last_insert_rowid(),
            name,
            description,
            price_cents: product.price_cents,
        })
    })
    .await
    .map_err(ApiError::internal)??;

    Ok((StatusCode::CREATED, Json(product)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::open("catalog.db")?;
    initialize_catalog(&connection)?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!(
        "Catalog service listening at http://{}",
        listener.local_addr()?
    );
    axum::serve(listener, app(connection)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Product, app, initialize_catalog};
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use rusqlite::Connection;
    use tower::ServiceExt;

    #[tokio::test]
    async fn catalog_get_returns_samples_and_post_adds_a_product() {
        let connection = Connection::open_in_memory().expect("open in-memory database");
        initialize_catalog(&connection).expect("initialize sample catalog");
        let app = app(connection);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/catalog")
                    .body(Body::empty())
                    .expect("build GET request"),
            )
            .await
            .expect("GET request should succeed");
        assert_eq!(response.status(), StatusCode::OK);
        let samples: Vec<Product> =
            serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap())
                .expect("decode sample catalog");
        assert_eq!(samples.len(), 3);
        assert_eq!(samples[0].name, "Coffee");

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/catalog")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"name":"Notebook","description":"Ruled paper notebook","price_cents":599}"#,
                    ))
                    .expect("build POST request"),
            )
            .await
            .expect("POST request should succeed");
        assert_eq!(response.status(), StatusCode::CREATED);
        let added: Product =
            serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap())
                .expect("decode created product");
        assert_eq!(added.name, "Notebook");
        assert_eq!(added.price_cents, 599);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/catalog")
                    .body(Body::empty())
                    .expect("build follow-up GET request"),
            )
            .await
            .expect("follow-up GET request should succeed");
        let products: Vec<Product> =
            serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap())
                .expect("decode updated catalog");
        assert_eq!(products.len(), 4);
        assert_eq!(products[3], added);
    }
}
