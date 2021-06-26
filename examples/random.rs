use postcode::Postcode;

#[async_std::main]
async fn main() {
    let post = Postcode::random().await.unwrap();

    println!("{} ({}, {}) -> ({}, {})", post.postcode, post.region, post.country, post.latitude, post.longitude);
}