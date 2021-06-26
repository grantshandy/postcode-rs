use postcode::Postcode;

#[async_std::main]
async fn main() {    
    let post = Postcode::from_coordinates(53.377476, -2.486197).await.unwrap();

    println!("{} ({}, {}) -> ({}, {})", post.postcode, post.region, post.country, post.latitude, post.longitude);
}