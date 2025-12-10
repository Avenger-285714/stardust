use serde::{Deserialize, Serialize};

/// Application data from Spark Store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    #[serde(rename = "Name")]
    pub name: String,
    
    #[serde(rename = "Desc")]
    pub desc: String,
    
    #[serde(rename = "Tagname")]
    pub tagname: Option<String>,
    
    #[serde(rename = "Contributor")]
    pub contributor: Option<String>,
    
    #[serde(rename = "Author")]
    pub author: Option<String>,
    
    #[serde(rename = "More")]
    pub more: Option<String>,
    
    #[serde(rename = "Pkg")]
    pub pkg: Option<String>,
    
    #[serde(rename = "Ver")]
    pub ver: Option<String>,
}

/// API client for Spark Store
#[derive(Clone)]
pub struct SparkStoreApi {
    base_url: String,
    arch_dir: String,
}

impl Default for SparkStoreApi {
    fn default() -> Self {
        Self::new()
    }
}

impl SparkStoreApi {
    pub fn new() -> Self {
        // Use Shandong University mirror (working as of 2024)
        let arch_dir = if cfg!(target_arch = "x86_64") {
            "amd64-store"
        } else if cfg!(target_arch = "aarch64") {
            "arm64-store"
        } else if cfg!(target_arch = "loongarch64") {
            "loong64-store"
        } else {
            "amd64-store" // default to x86_64
        };
        
        Self {
            base_url: "https://mirrors.sdu.edu.cn/spark-store-repository/".to_string(),
            arch_dir: arch_dir.to_string(),
        }
    }
    
    /// Fetch application list for a category
    pub async fn fetch_app_list(&self, category: &str) -> Result<Vec<AppInfo>, String> {
        // Validate category to prevent path traversal
        if category.contains("..") || category.contains('/') || category.contains('\\') {
            return Err("Invalid category name".to_string());
        }
        
        let url = format!("{}{}/{}/applist.json", self.base_url, self.arch_dir, category);
        
        eprintln!("Fetching app list from: {}", url);
        
        // Create client with timeout
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
        
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    format!("Request timed out after 30 seconds: {}", url)
                } else if e.is_connect() {
                    format!("Failed to connect to server (check network connection): {}", url)
                } else if e.to_string().contains("dns") || e.to_string().contains("resolve") {
                    format!("DNS resolution failed (server may be unavailable): {}", url)
                } else {
                    format!("Network error: {} - URL: {}", e, url)
                }
            })?;
        
        if !response.status().is_success() {
            return Err(format!("Server returned error {}: {}", response.status(), url));
        }
        
        let apps: Vec<AppInfo> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON from {}: {}", url, e))?;
        
        eprintln!("Successfully loaded {} apps from category: {}", apps.len(), category);
        
        Ok(apps)
    }
    
    /// Search for applications
    pub async fn search_apps(&self, keyword: &str) -> Result<Vec<AppInfo>, String> {
        // Fallback: search by fetching all apps and filtering locally
        // This is more reliable than depending on external search API
        eprintln!("Searching locally for: {}", keyword);
        
        // Fetch all apps from the "all" category
        let all_apps = self.fetch_app_list("all").await?;
        
        let keyword_lower = keyword.to_lowercase();
        let filtered: Vec<AppInfo> = all_apps
            .into_iter()
            .filter(|app| {
                app.name.to_lowercase().contains(&keyword_lower)
                    || app.desc.to_lowercase().contains(&keyword_lower)
            })
            .collect();
        
        eprintln!("Search found {} results for: {}", filtered.len(), keyword);
        
        Ok(filtered)
    }
}
