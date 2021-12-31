use actix_web::{get, web, HttpResponse, http};
use crate::database::{Database, DatabaseError};

#[get("/{url_id}")]
pub async fn redirect(
    web::Path(url_id): web::Path<String>,
    database: web::Data<Database>,
) -> HttpResponse {
    match database.get_url_entry(&url_id) {
        Ok(url_entry) => {
            return HttpResponse::Found()
                .header(http::header::LOCATION, url_entry.url)
                .finish();
        }
        Err(err) => {
            if let Some(database_err) = err.downcast_ref::<DatabaseError>() {
                return match &database_err {
                    DatabaseError::NotFound { .. } => {
                        HttpResponse::NotFound()
                            .body("404 url not found")
                    }
                };
            }

            return HttpResponse::InternalServerError().finish();
        }
    }
}
