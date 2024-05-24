use axum::Json;
//use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MirrorJson {
    name: String,
    age: isize,
    message: String,
}

#[derive(Serialize,Deserialize)]
pub struct MirrorJsonResonse {
    your_message: String,
    messahe_from_server: String,
}

pub async fn mirror_body_json(Json(body):Json<MirrorJson>) ->Json<MirrorJsonResonse> {
    dbg!(body.clone());
    Json(MirrorJsonResonse{
        your_message:body.message,
        messahe_from_server: format!("Hello {}, from Axum server.", body.name.to_owned()).to_owned(),
    })
    
}