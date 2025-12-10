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
    mirrors: Vec<String>,
    arch_dir: String,
}

impl Default for SparkStoreApi {
    fn default() -> Self {
        Self::new()
    }
}

impl SparkStoreApi {
    pub fn new() -> Self {
        // Multiple mirrors for redundancy (as of December 2024)
        let mirrors = vec![
            "https://mirrors.sdu.edu.cn/spark-store-repository/".to_string(),
            "https://mirrors.sdu.edu.cn/spark-store/".to_string(),
            "https://gitee.com/spark-store-project/spark-store/raw/master/".to_string(),
        ];
        
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
            mirrors,
            arch_dir: arch_dir.to_string(),
        }
    }
    
    /// Fetch application list for a category
    pub async fn fetch_app_list(&self, category: &str) -> Result<Vec<AppInfo>, String> {
        // Validate category to prevent path traversal
        if category.contains("..") || category.contains('/') || category.contains('\\') {
            return Err("Invalid category name".to_string());
        }
        
        // Try each mirror in order until one succeeds
        let mut last_error = String::new();
        
        for (i, base_url) in self.mirrors.iter().enumerate() {
            let url = format!("{}{}/{}/applist.json", base_url, self.arch_dir, category);
            
            eprintln!("[Attempt {}/{}] Fetching app list from: {}", i + 1, self.mirrors.len(), url);
            
            // Create client with timeout
            let client = match reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
            {
                Ok(c) => c,
                Err(e) => {
                    last_error = format!("Failed to create HTTP client: {}", e);
                    eprintln!("  ✗ {}", last_error);
                    continue;
                }
            };
            
            match client.get(&url).send().await {
                Ok(response) => {
                    if !response.status().is_success() {
                        last_error = format!("Server returned error {}", response.status());
                        eprintln!("  ✗ {}", last_error);
                        continue;
                    }
                    
                    match response.json::<Vec<AppInfo>>().await {
                        Ok(apps) => {
                            eprintln!("  ✓ Successfully loaded {} apps from category: {}", apps.len(), category);
                            return Ok(apps);
                        }
                        Err(e) => {
                            last_error = format!("Failed to parse JSON: {}", e);
                            eprintln!("  ✗ {}", last_error);
                            continue;
                        }
                    }
                }
                Err(e) => {
                    if e.is_timeout() {
                        last_error = format!("Request timed out after 30 seconds");
                    } else if e.is_connect() {
                        last_error = format!("Failed to connect to server");
                    } else if e.to_string().contains("dns") || e.to_string().contains("resolve") {
                        last_error = format!("DNS resolution failed");
                    } else {
                        last_error = format!("Network error: {}", e);
                    }
                    eprintln!("  ✗ {}", last_error);
                    continue;
                }
            }
        }
        
        // All mirrors failed
        Err(format!(
            "Failed to fetch app list after trying {} mirror(s). Last error: {}. \
            Check your internet connection and firewall settings.",
            self.mirrors.len(),
            last_error
        ))
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
