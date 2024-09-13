pub mod tests {
    use reqwest::StatusCode;

    #[tokio::test]
    pub async fn status_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/status");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    // Supplies/Toner

    #[tokio::test]
    pub async fn toner_count_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/supplies/toner-count");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn toner_search_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/supplies/toner/wrong_id");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    pub async fn show_toners_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/supplies/toners");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_toner_endpoint() {
        let body = r#"{
        "name": "TEST Toner",
        "color": "wrong_color"
    }"#;

        let endpoint = "http://localhost:8000/api/v1/supplies/toners";

        let client = reqwest::Client::new();

        let response = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn update_toner_endpoint() {
        let body = r#"{
        "id": "wrong_id",
        "name": "TEST Toner",
        "color": "black",
    }"#;

        let endpoint = "http://localhost:8000/api/v1/supplies/toners";

        let client = reqwest::Client::new();

        let response = client
            .put(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_toner_endpoint() {
        let body = r#"{
        "id": "wrong_id"
    }"#;

        let endpoint = "http://localhost:8000/api/v1/supplies/toners";

        let client = reqwest::Client::new();

        let response = client
            .delete(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // Supplies/Drum

    #[tokio::test]
    pub async fn drum_count_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/supplies/drum-count");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn drum_search_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/supplies/drum/wrong_id");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    pub async fn show_drums_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/supplies/drums");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_drum_endpoint() {
        let body = r#"{
        "name--": "TEST Drum"
    }"#;

        let endpoint = "http://localhost:8000/api/v1/supplies/drums";

        let client = reqwest::Client::new();

        let response = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn update_drum_endpoint() {
        let body = r#"{
        "id": "wrong_id",
        "name": "TEST Drum",
    }"#;

        let endpoint = "http://localhost:8000/api/v1/supplies/drums";

        let client = reqwest::Client::new();

        let response = client
            .put(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_drum_endpoint() {
        let body = r#"{
        "id": "wrong_id"
    }"#;

        let endpoint = "http://localhost:8000/api/v1/supplies/drums";

        let client = reqwest::Client::new();

        let response = client
            .delete(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // Brand

    #[tokio::test]
    pub async fn brand_count_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/brand-count");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn brand_search_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/brand/wrong_id");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    pub async fn show_brands_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/brands");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_brand_endpoint() {
        let body = r#"{
        "name--": "TEST Brand"
    }"#;

        let endpoint = "http://localhost:8000/api/v1/brands";

        let client = reqwest::Client::new();

        let response = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn update_brand_endpoint() {
        let body = r#"{
        "id": "wrong_id",
        "name": "TEST brand",
    }"#;

        let endpoint = "http://localhost:8000/api/v1/brands";

        let client = reqwest::Client::new();

        let response = client
            .put(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_brand_endpoint() {
        let body = r#"{
        "id": "wrong_id"
    }"#;

        let endpoint = "http://localhost:8000/api/v1/brands";

        let client = reqwest::Client::new();

        let response = client
            .delete(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // Printer

    #[tokio::test]
    pub async fn printer_count_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/printer-count");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn printer_search_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/printer/wrong_id");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    pub async fn show_printers_endpoint() {
        let endpoint = format!("http://localhost:8000/api/v1/printers");
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_printer_endpoint() {
        let body = r#"{
        "name": "TEST printer",
        "model": "TEST model",
        "brand": "wrong_id",
        "toner": "wrong_id",
        "drum": "wrong_id",
    }"#;

        let endpoint = "http://localhost:8000/api/v1/printers";

        let client = reqwest::Client::new();

        let response = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn update_printer_endpoint() {
        let body = r#"{
            "id": "wrong_id",
            "name": "TEST printer",
            "model": "TEST model",
            "brand": "wrong_id",
            "toner": "wrong_id",
            "drum": "wrong_id",
        }"#;

        let endpoint = "http://localhost:8000/api/v1/printers";

        let client = reqwest::Client::new();

        let response = client
            .put(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_printer_endpoint() {
        let body = r#"{
        "id": "wrong_id"
    }"#;

        let endpoint = "http://localhost:8000/api/v1/printers";

        let client = reqwest::Client::new();

        let response = client
            .delete(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}