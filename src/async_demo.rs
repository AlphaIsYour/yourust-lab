use reqwest;

pub fn run() {
    // Karena Rust async perlu runtime, kita buat main async di sini
    // pakai tokio::runtime::Runtime
    println!("--- Async Demo (HTTP request) ---");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match reqwest::get("https://httpbin.org/get").await {
            Ok(resp) => {
                let text = resp.text().await.unwrap();
                println!("Response from httpbin.org:\n{}", text);
            },
            Err(e) => {
                println!("Error fetching URL: {}", e);
            }
        }
    });
}
