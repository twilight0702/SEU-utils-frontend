// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn fetch_grades(cookie: String) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .post("https://ehall.seu.edu.cn/jwapp/sys/cjcx/modules/cjcx/xscjcx.do")
        .header("Cookie", cookie)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 Edg/123.0.0.0")
        .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
        .header("Referer", "https://ehall.seu.edu.cn/")
        .header("X-Requested-With", "XMLHttpRequest")
        .body("")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    let _headers = response.headers().clone();  // 克隆头信息
    
    // 先处理响应内容
    if status.is_success() {
        let json = response.json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(json)
    } else {
        let text = response.text()
            .await
            .unwrap_or_else(|_| "Failed to get error text".to_string());
        Err(format!("HTTP {}: {}", status, text))
    }
}

#[tauri::command]
async fn fetch_grades2(cookie: String, url: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    match client.post(&url)
        .header("Cookie", cookie)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36 Edg/133.0.0.0")
        .header("Content-Length", "0")
        .send()
        .await
    {
        Ok(res) => {
            let text = res.text().await.map_err(|e| e.to_string())?;
            Ok(text)
        }
        Err(e) => Err(e.to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_grades,fetch_grades2]) // 注册命令
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}