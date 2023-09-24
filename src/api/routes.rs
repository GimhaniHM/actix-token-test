use crate::{repository::mongodb_repo::MongoRepo, 
    models::user_model::{
        User,
        LoginUserSchema,
        TokenClaims
    }, AppState,
    //middleware::auth::AuthorizationService,
};

use actix_web::{
    post, get ,
    web::{Data, Json, self},
    HttpResponse, Responder, cookie::Cookie,
    cookie::time::Duration as ActixWebDuration, HttpRequest, HttpMessage,
};



use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde_json::json;

//route handler function
#[get("/healthchecker")]
async fn index() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, mongodb";
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}


//register user
#[post("/user")]
pub async fn register_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        pwd: new_user.pwd.to_owned(),
        email: new_user.email.to_owned(),
        location: new_user.location.to_owned(),
    };

    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[post("/login/{id}")]
async fn login_user(path: web::Path<String>) -> impl Responder {
   
    let jwt_secret = "secret";

    let id = path.into_inner();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: id,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(register_user)
        .service(login_user);
}