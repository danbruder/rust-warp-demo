use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::http::StatusCode;

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

type State = Arc<Mutex<DogMap>>;

#[tokio::main]
async fn main() {
    // Add one dog for testing.
    let id = Uuid::new_v4().to_string();
    let dog = Dog {
        id: id.clone(),
        name: "Comet".to_string(),
        breed: "Whippet".to_string(),
    };
    let mut dog_map = HashMap::new();
    dog_map.insert(id, dog);

    let state: State = Arc::new(Mutex::new(dog_map));
    let cloned_state = warp::any().map(move || state.clone());

    let get_dogs =
        warp::path!("dog")
            .and(warp::get())
            .and(cloned_state.clone())
            .map(|dog_map: DogMap| {
                println!("got get: dog_map = {:?}", dog_map);
                let dogs: Vec<Dog> = dog_map.values().cloned().collect();
                Ok(warp::reply::json(&dogs))
            });

    let get_dog = warp::path!("dog" / String)
        .and(warp::get())
        .and(cloned_state.clone())
        .map(|id, dog_map: DogMap| {
            println!("got get for id {}, dog_map = {:?}", id, dog_map);
            if let Some(dog) = dog_map.get(&id) {
                Ok(warp::reply::json(&dog))
            } else {
                Err(warp::reject::not_found())
            }
        });

    let create_dog =
        warp::path!("dog")
            .and(warp::post())
            .and(warp::body::json())
            .and(cloned_state.clone())
            .map(|new_dog: NewDog, dog_map: DogMap| {
                println!("got post request with {:?}", new_dog);
                let id = Uuid::new_v4().to_string();
                let dog = Dog { id, name: new_dog.name, breed: new_dog.breed};
                dog_map.insert(id, dog);
                //Ok(warp::reply::with_status("success", StatusCode::CREATED))
                Ok(warp::reply::json(&dog))
            });

    let update_dog =
        warp::path!("dog" / String)
            .and(warp::put())
            .and(warp::body::json())
            .and(cloned_state.clone())
            .map(|id: String, dog: Dog, dog_map: DogMap| {
                println!("got put request with id {} and {:?}", id, dog);
                if let Some(dog) = dog_map.get(&id) {
                    dog_map.insert(id, dog);
                    Ok(warp::reply::json(&dog))
                } else {
                    Err(warp::reject::not_found())
                }
            });

    let delete_dog =
        warp::path!("dog" / String)
            .and(warp::delete())
            .and(cloned_state.clone())
            .map(|id: String, dog_map: DogMap| {
                println!("got delete request with id {}", &id);
                if let Some(_dog) = dog_map.remove(&id) {
                    Ok(warp::reply::with_status("success", StatusCode::OK))
                } else {
                    Ok(warp::reply::with_status("dog not found", StatusCode::NOT_FOUND))
                }
            });

    //TODO: Learn how to get this to use TLS/HTTPS.
    let routes = get_dogs.or(get_dog).or(create_dog).or(update_dog).or(delete_dog);
    warp::serve(routes).run(([127, 0, 0, 1], 1234)).await;
}
