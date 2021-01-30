use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
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

type State = Arc<RwLock<DogMap>>;

#[tokio::main]
async fn main() {
    let state: State = Arc::new(RwLock::new(HashMap::new()));

    // Add one dog for testing.
    let id = Uuid::new_v4().to_string();
    let dog = Dog {
        id: id.clone(),
        name: "Comet".to_string(),
        breed: "Whippet".to_string(),
    };
    state.write().insert(id, dog);

    //let dog_map_state = warp::any().map(move || dog_map.clone());

    // Browse localhost:8000/dog/{id}
    let get_dog = warp::path!("dog" / String)
        .and(warp::get())
        .map(|id| {
            /*
            if let Some(dog) = dog_map.read().get(&id).as_ref() {
                Ok(warp::reply::json(&dog))
            } else {
                Err(warp::reject::not_found())
            }
            */
            println!("got get for id {}", id);
            Ok("not implemented")
        });

    // Browse localhost:8000/dog
    let get_dogs =
        warp::path!("dog")
            .and(warp::get())
            //.and(state)
            //.map(|state: Arc<RwLock<HashMap<_, _>>>| {
            .map(|| {
                //let dogs: Vec<Dog> = state.read().values().cloned().collect();
                //warp::reply::json(&dogs)
                println!("got get for all dogs");
                Ok("got get for all dogs")
            });

    /*
    async fn create_dog(dog: Dog, state: State) {
        let id = Uuid::new_v4().to_string();
        dog.id = id;
        state.write.insert(id, dog);
        Ok(warp::reply::with_status("success", http::StatusCode::CREATED))
    }
    */

    let create_dog =
        warp::path!("dog")
            .and(warp::post())
            .and(warp::body::json())
            .map(|dog: NewDog| {
                //|serde_json::value, arc: Arc<RwLock<HashMap<_, _>>>| async move {
                println!("got post request with {:?}", dog);
                Ok("got post")
            });

    let update_dog =
        warp::path!("dog" / String)
            .and(warp::put())
            .and(warp::body::json())
            .map(|id: String, dog: Dog| {
                 println!("got put request with id {} and {:?}", id, dog);
                 Ok("got put")
            });

    let delete_dog =
        warp::path!("dog" / String)
            .and(warp::delete())
            .map(|id: String| {
                println!("got delete request with id {}", id);
                Ok("got delete")
            });

    //TODO: Learn how to get this to use TLS/HTTPS.
    let routes = get_dogs.or(get_dog).or(create_dog).or(update_dog).or(delete_dog);
    warp::serve(routes).run(([127, 0, 0, 1], 1234)).await;
}
