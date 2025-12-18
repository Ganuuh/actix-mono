use actix_web::HttpResponse;

#[derive(serde::Serialize)]
struct ApplicationResponse<T> {
    pub data: T,
    pub message: Option<String>,
}

pub fn presenter<T: serde::Serialize>(body: T, message: Option<String>) -> HttpResponse {
    HttpResponse::Ok().json(ApplicationResponse {
        data: body,
        message,
    })
}
