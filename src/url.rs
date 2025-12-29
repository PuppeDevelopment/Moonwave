#[derive(Debug, Clone)]
pub struct Url {
    pub host: String,
    pub path_and_query: String,
}

impl Url {
    pub fn parse_url(url: &str) -> Self {
        let mut host = String::new();
        let mut path_and_query = String::new();
    
        if url.is_empty() {
            return Url { host, path_and_query };
        }
    
        if let Some(proto_end) = url.find("://") {
            host.push_str(&url[..proto_end]); // protocol
            host.push_str("://");
            let remainder = &url[(proto_end + 3)..];
    
            if let Some(path_start) = remainder.find('/') {
                host.push_str(&remainder[..path_start]); // address/domain + port
                path_and_query.push_str(&remainder[path_start..]); // path + query
            } else {
                host.push_str(remainder);
            }
        } else {
            // No protocol ?????? (shouldnt happen btw)
            if let Some(path_start) = url.find('/') {
                host.push_str(&url[..path_start]);
                path_and_query.push_str(&url[path_start..]);
            } else {
                host.push_str(url);
            }
        }
    
        Url { host, path_and_query }
    }
    
    // why i love rust frfr
    pub fn should_redirect(host: &str) -> bool {
        let host = host
            .strip_prefix("http://")
            .or_else(|| host.strip_prefix("https://"))
            .unwrap_or(host);
    
        let host = host.split(':').next().unwrap_or(host);
    
        const EPIC_DOMAINS: [&str; 6] = [
            "game-social.epicgames.com",
            "ol.epicgames.com",
            "ol.epicgames.net",
            "on.epicgames.com",
            "ak.epicgames.com",
            "epicgames.dev",
        ];
    
        EPIC_DOMAINS.iter().any(|d| host.ends_with(d))
    }    
    
    pub fn create_url(host: &str, path_and_query: &str) -> String {
        let mut url = String::with_capacity(host.len() + path_and_query.len());
        url.push_str(host);
        url.push_str(path_and_query);
        url
    }    
}