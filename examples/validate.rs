#[async_std::main]
async fn main() {
    let code_1 = "SW1W0NY";
    let code_2 = "WA5B9ZL";

    let res_1 = postcode::validate(code_1).await.unwrap();
    let res_2 = postcode::validate(code_2).await.unwrap();

    println!("{}: {}", code_1, res_1);
    println!("{}: {}", code_2, res_2);
}