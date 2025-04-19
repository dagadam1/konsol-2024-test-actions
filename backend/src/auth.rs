use actix_session::SessionExt;
use actix_web::{error, web};
use serde::{Deserialize, Serialize};

use crate::actions;

use super::DbPool;

use std::future::{ready, Ready};

use actix_web::FromRequest;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum PermissionLevel {
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuthenticatedUser {
    pub(crate) email: String,
    pub(crate) permission: PermissionLevel,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let session = req.get_session();

        match session.get::<AuthenticatedUser>("auth") {
            Ok(Some(user)) => {
                ready(Ok(user)) 
            },
            _ => ready(Err(actix_web::error::ErrorUnauthorized("Not logged in"))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AuthRequest {
    pub(crate) client_id: String,
    pub(crate) id_token: String,
}

pub(crate) async fn check_user_permission(email: String, pool: web::Data<DbPool>) -> actix_web::Result<Option<PermissionLevel>> {
    web::block(move || {
        let mut conn = pool.get()?;
        actions::check_email_permission(&mut conn, &email)
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}
