use axum::{body::Body, http::Request, Json};
use bcrypt::{hash, DEFAULT_COST};
use bson::oid::ObjectId;
use bson::Document;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
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

pub fn bson_to_json(bson_doc: Document) -> Value {
    let json_string = serde_json::to_string(&bson_doc).unwrap();
    let json_value: Value = serde_json::from_str(&json_string).unwrap();
    json_value
}

async fn test_mongo() -> Result<Document, Box<dyn Error>> {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    // Get the 'movies' collection from the 'sample_mflix' database:
    let test = client.database("Auth").collection("User");

    let test: Document = test
        .find_one(
            doc! {
                  "name": "bob sacamano"
            },
            None,
        )
        .await?
        .expect("Cannot find document");

    Ok(test)
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

pub async fn get_user() -> Json<Value> {
    let response = test_mongo().await.expect("didn't work");
    let response_json = bson_to_json(response);
    Json(json!({
        "data": response_json,
        "success": true
    }))
    // Json(json!())
}

pub async fn get_self(request: Request<Body>) -> Json<Value> {
    let headers = request.headers();
    let authorization = headers
        .get("authorization")
        .unwrap()
        .to_str()
        .expect("could not get header");

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let claims: BTreeMap<String, String> = authorization.verify_with_key(&key).unwrap();
    let id = claims["id"].to_string();
    let object_id = ObjectId::parse_str(&id).expect("could not get objectid");

    let user_db = user_db().await.expect("could not connect to db");

    let filter = doc! {"_id": object_id };
    let user = user_db
        .find_one(filter, None)
        .await
        .expect("could not find user")
        .unwrap();

    let user_json = bson_to_json(user);

    Json(user_json)
}

#[derive(Deserialize)]
pub struct CreateUser {
    email: String,
    password: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> Json<Value> {
    let hash = hash(payload.password, DEFAULT_COST).expect("could not hash");

    let user_db = user_db().await.expect("Could not connect to db");

    let new_doc = doc! {
       "email": &payload.email,
       "password": &hash
    };

    let result = user_db
        .insert_one(new_doc.clone(), None)
        .await
        .expect("could not create new record");

    let id = &result.inserted_id;

    Json(json!({ "id": id }))
}
