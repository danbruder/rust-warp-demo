use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const BASE_URL: &str = "http://localhost:1234/dog";

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Dog {
    id: String,
    breed: String,
    name: String,
}

async fn assert_dog_count(client: &Client, count: usize) -> Result<Vec<Dog>, Box<dyn std::error::Error>> {
    let dogs = get_all_dogs(client).await?;
    assert_eq!(dogs.len(), count);
    Ok(dogs)
}

async fn get_all_dogs(client: &Client) -> Result<Vec<Dog>, Box<dyn std::error::Error>> {
    let res = client.get(BASE_URL).send().await?;
    let dogs = res.json::<Vec<Dog>>().await?;
    Ok(dogs)
}

async fn delete_all_dogs(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let dogs = get_all_dogs(client).await?;
    for dog in dogs {
        let url = format!("{}/{}", BASE_URL, dog.id);
        client.delete(&url).send().await?.text().await?;
    }
    Ok(())
}

#[tokio::test]
async fn it_uses_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let count: usize = 2;
    let client = reqwest::Client::new();

    // Delete all the current dogs to start with an empty collection.
    delete_all_dogs(&client).await?;
    assert_dog_count(&client, 0).await?;

    // Create new dogs.
    for i in 0..count {
        let id = Uuid::new_v4().to_string();
        let dog = Dog {
            id,
            name: format!("name-{}", i),
            breed: format!("breed-{}", i)
        };
        client.post(BASE_URL).json(&dog).send().await?;
    }
    let dogs = assert_dog_count(&client, count).await?;
    for dog in &dogs {
        assert!(dog.name.starts_with("name-"));
        assert!(dog.breed.starts_with("breed-"));
    }

    // Update all the dogs.
    let prefix = "new-";
    for dog in dogs {
        let id = dog.id.clone();
        let new_dog = Dog {
            id: id.clone(),
            name: format!("{}{}", prefix, dog.name),
            breed: format!("{}{}", prefix, dog.breed)
        };
        let url = format!("{}/{}", BASE_URL, id);
        client.put(&url).json(&new_dog).send().await?;
    }
    let dogs = assert_dog_count(&client, count).await?;
    let name_prefix = format!("{}name-", prefix);
    let breed_prefix = format!("{}breed-", prefix);
    for dog in dogs {
        assert!(dog.name.starts_with(&name_prefix));
        assert!(dog.breed.starts_with(&breed_prefix));
    }

    delete_all_dogs(&client).await?;
    assert_dog_count(&client, 0).await?;

    Ok(())
}