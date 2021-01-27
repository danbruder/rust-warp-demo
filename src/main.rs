use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc};
use uuid::Uuid;
use warp::Filter;

// We need to implement the "Clone" trait in order to
// call the "cloned" method in the "get_dogs" route.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Dog {
    id: String,
    breed: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct NewDog {
    breed: String,
    name: String,
}

type DogMap = HashMap<String, Dog>;

type SafeDogMap = Arc<RwLock<DogMap>>;

#[tokio::main]
async fn main() {
    let dog_map: SafeDogMap = Arc::new(RwLock::new(HashMap::new()));

    // Add one dog for testing.
    let id = Uuid::new_v4().to_string();
    let dog = Dog {
        id: id.clone(),
        name: "Comet".to_string(),
        breed: "Whippet".to_string(),
    };
    dog_map.write().insert(id, dog);

    // This is just for verifying that Warp is working.
    // Browse localhost:8000/hello/April/21
    let hello = warp::path!("hello" / String / u8)
        .map(|name, age| format!("Hello, {} year old named {}!", age, name));

    // Browse localhost:8000/dog/{id}
    let get_dog = warp::path!("dog" / String).and_then(move |id: String| {
        if let Some(dog) = dog_map.read().get(&id) {
            Ok(warp::reply::json(&dog))
        } else {
            Err(warp::reject::not_found())
        }
    });

    // Browse localhost:8000/dog
    let get_dogs = warp::path!("dog").map(move || {
        let dogs: Vec<Dog> = dog_map.read().values().cloned().collect();
        warp::reply::json(&dogs)
    });


    //TODO: Learn how to get this to use TLS/HTTPS.
    let routes = hello.or(get_dogs).or(get_dog);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
