use postcode::Postcode;

#[async_std::main]
async fn main() {
    let code = "SW1W0NY";
    
    let post = Postcode::from_code(code).await.unwrap();

    println!("{} ({}, {}) -> ({}, {})", code, post.region, post.country, post.latitude, post.longitude);
}