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

impl AppInfo {
    pub fn display_name(&self) -> String {
        self.name.clone()
    }
    
    pub fn display_desc(&self) -> String {
        self.desc.clone()
    }
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
        let arch_dir = if cfg!(target_arch = "x86_64") {
            "store"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64-store"
        } else if cfg!(target_arch = "loongarch64") {
            "loong64-store"
        } else {
            "store" // default to x86_64
        };
        
        Self {
            base_url: "https://cdn-d.spark-app.store/".to_string(),
            arch_dir: arch_dir.to_string(),
        }
    }
    
    /// Fetch application list for a category
    pub async fn fetch_app_list(&self, category: &str) -> Result<Vec<AppInfo>, String> {
        let url = format!("{}{}/{}/applist.json", self.base_url, self.arch_dir, category);
        
        let response = reqwest::get(&url)
            .await
            .map_err(|e| format!("Failed to fetch app list: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("Server returned error: {}", response.status()));
        }
        
        let apps: Vec<AppInfo> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        Ok(apps)
    }
    
    /// Search for applications
    pub async fn search_apps(&self, keyword: &str) -> Result<Vec<AppInfo>, String> {
        let url = format!("https://search.deepinos.org.cn/appinfo/search?keyword={}", keyword);
        
        let response = reqwest::get(&url)
            .await
            .map_err(|e| format!("Failed to search: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("Search failed: {}", response.status()));
        }
        
        let apps: Vec<AppInfo> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse search results: {}", e))?;
        
        Ok(apps)
    }
}
