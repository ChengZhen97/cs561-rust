mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A hard-coded JSON
    let json = r#"
            {
              "main": {
                "temp": 30.94
              }
            }
        "#;

    // Deserialize the hardcoded JSON into a Weather struct
    let weather1: model::Weather = serde_json::from_str(json).unwrap();

    println!("\nWeather from a JSON we hard-coded locally:\n{:?}", weather1);

    //
    // Now that we know we can deserialize a hard-coded JSON into a struct model,
    // let's see if we can fetch the weather from the backend.
    //
    //record the username and password
    let mut map = HashMap::new();
    map.insert("username", "usr");
    map.insert("password", "pass");
    //first get authenticated
    let clientAuth = reqwest::Client::new();
    let res = clientAuth.post("http://localhost:3000/v1/auth")
        .json(&map)
        .send()
        .await?;
    let auth_token = res
        .json::<model::Token>()
        .await?;
    println!("\nToken from backend service:\n {:?}\n", auth_token);
    let header_value = format!("Bearer {}", auth_token.accessToken);
    //get weather client
    let client = reqwest::Client::new();

    let response = client.get("http://localhost:3000/v1/weather")
        .header(AUTHORIZATION, header_value)
        .send()
        .await?;

    let weather2 = response
        .json::<model::Weather>()
        .await?;

    println!("\nWeather from openweathermap.org:\n {:?}", weather2);

    Ok(())
}
