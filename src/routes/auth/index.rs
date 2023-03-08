use axum::Json;
use bcrypt::verify;
use bson::Document;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;

#[derive(Deserialize)]
pub struct User {
    email: String,
    password: String,
}

pub fn bson_to_json(bson_doc: Document) -> Value {
    let json_string = serde_json::to_string(&bson_doc).unwrap();
    let json_value: Value = serde_json::from_str(&json_string).unwrap();
    json_value
}

async fn user_db() -> Result<Collection<Document>, Box<dyn Error>> {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    // Get the 'movies' collection from the 'sample_mflix' database:
    let test: Collection<Document> = client.database("Auth").collection("User");

    Ok(test)
}

#[derive(Deserialize)]
pub struct SignIn {
    email: String,
    password: String,
}

pub async fn sign_in(Json(payload): Json<SignIn>) -> Json<Value> {
    let user_db = user_db().await.expect("Could not connect to db");

    let user = user_db
        .find_one(
            doc! {
                  "email": payload.email
            },
            None,
        )
        .await
        .expect("Cannot find document")
        .unwrap();

    let hashed_password = user.get_str("password").expect("could not get password");
    let id = user
        .get_object_id("_id")
        .expect("could not get id")
        .to_hex();
    println!("{id}");

    let valid = verify(payload.password, &hashed_password).expect("could not check password");

    if valid == false {
        panic!("Invalid password")
    }

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").expect("could not generate key");
    let mut claims = BTreeMap::new();
    claims.insert("id", id);
    let token_str = claims.sign_with_key(&key).expect("could not sign key");

    Json(json!({ "valid": valid, "jwt": token_str }))
}
