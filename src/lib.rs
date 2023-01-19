use actix_web::error::ResponseError;
use macros_create_app::make_app71;
use macros_make_error::make_error2;
use macros_make_model::make_model23;
use macros_make_model::make_model35;
use macros_make_scope::make_scope1;
use serde::*;
use serde_repr::{Deserialize_repr, Serialize_repr};
use url::Url;

make_error2!(ProductError);

make_model23!(QProductPrice, IProductPrice, OProductPrice, name: String,);

make_model35!(
    Q,
    I,
    O,
    product,
    name: String,
    price: bigdecimal::BigDecimal,
    image: String,
    pid: String,
    uid: String,
    currency: String,
    url: String,
    prices: sqlx::types::Json<Vec<IProductPrice>>,
    macros: Option<sqlx::types::Json<IProductMacros>>,
    price_list: Vec<String>,
    size_list: Vec<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    search: String,
    indexed_at: Option<chrono::DateTime<chrono::Utc>>
);

make_app71!(
    [
        name: String,
        price: bigdecimal::BigDecimal,
        image: String,
        pid: String,
        uid: String,
        currency: String,
        url: String,
        prices: sqlx::types::Json<Vec<IProductPrice>>,
        macros: Option<sqlx::types::Json<IProductMacros>>,
        price_list: Vec<String>,
        size_list: Vec<String>,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
        search: String,
        indexed_at: Option<chrono::DateTime<chrono::Utc>>
    ],
    route,
    "/product",
    "/product/{id}",
    "",
    "{id}",
    O,
    Q,
    I,
    product,
    [
        |s: actix_web::web::Data<my_state::MyState>,
         json: actix_web::web::Json<route::IRequest>,
         wallet: lib_wallet::QWallet,
         http_request: actix_web::HttpRequest| async move { handle(s, json, wallet).await }
    ],
    ProductError
);

make_scope1!("product", [put, route]);

async fn handle(
    s: actix_web::web::Data<my_state::MyState>,
    json: actix_web::web::Json<route::IRequest>,
    _: lib_wallet::QWallet,
) -> Result<Q, ProductError> {
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        tracing::info!("good day mat");
        ()
    });
    if json.data.search.is_empty() {
        Ok(Q::default())
    } else {
        product::postgres_query::insert(&s.sqlx_pool, &json.data)
            .await
            .map_err(ProductError::from_general)
    }
}
