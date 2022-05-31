
#[test]
fn test_add() {
    assert_eq!(3 + 2, 5);
}

#[actix_rt::test]
async fn test_reqwest_failed() {
    let client = reqwest::Client::new();

    let result = client.post("http://ewnfjbef:8080/compile").send().await;
    assert!(!result.unwrap_err().is_status());
}