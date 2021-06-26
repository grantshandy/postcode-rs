use postcode::Postcode;

#[async_std::main]
async fn main() {
    let v = vec!["PL8 1JN", "SW4 6QT", "OL4 2HJ"];
    let x = Postcode::from_multi_lookup(v).await.unwrap();

    for y in x {
        println!("{} ({}, {}) -> ({}, {})", y.postcode, y.region, y.country, y.latitude, y.longitude);
    }
}