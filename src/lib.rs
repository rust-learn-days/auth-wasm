use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use base64;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpHeadersRoot) });
}}

struct HttpHeadersRoot;

impl Context for HttpHeadersRoot {}

impl RootContext for HttpHeadersRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpHeaders { context_id }))
    }
}

struct HttpHeaders {
    context_id: u32,
}

impl Context for HttpHeaders {}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        // 获取请求头中的Authorization字段，该字段包含Basic认证凭据
        if let Some(auth_header) = self.get_http_request_header("Authorization") {
            // 检查Basic认证凭据是否合法（用户名和密码是否匹配）
            if let Some(credentials) = extract_basic_auth_credentials(&auth_header) {
                let (username, password) = credentials;
                // 验证用户名和密码是否为"user"和"user123"
                if username == "user" && password == "user123" {
                    // 符合条件，继续执行请求处理
                    for (name, value) in &self.get_http_request_headers() {
                        info!("#{} -> {}: {}", self.context_id, name, value);
                    }
                    let auth_header = encode_basic_auth_credentials("admin", "passw0rd");
                    self.set_http_request_header("X-AUTH-WASM", Some("wasm32-wasi"));
                    self.set_http_request_header("Authorization",Some(&auth_header));
                    return Action::Continue;
                }
            }
        }
		let body: &str = &response_401_body();
        // 未通过Basic认证或用户名密码不匹配，返回401 Unauthorized
        self.send_http_response(
            401,
            vec![("WWW-Authenticate", "Basic realm=\"Registry Realm\""),("Docker-Distribution-Api-Version","registry/2.0"),("Content-Type","application/json; charset=utf-8")],
            Some(body.as_ref()),

        );
        Action::Pause
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        for (name, value) in &self.get_http_response_headers() {
            info!("#{} <- {}: {}", self.context_id, name, value);
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        info!("#{} completed.", self.context_id);
    }
}

// 提取Basic认证凭据中的用户名和密码
fn extract_basic_auth_credentials(auth_header: &str) -> Option<(&'static str, &'static str)> {
    const BASIC_PREFIX: &str = "Basic ";
    if auth_header.starts_with(BASIC_PREFIX) {
        let encoded_credentials = &auth_header[BASIC_PREFIX.len()..].trim();
        if let Ok(credentials) = base64::decode(encoded_credentials) {
            if let Ok(credentials_str) = std::str::from_utf8(&credentials) {
                if let Some(index) = credentials_str.find(':') {
                    let (username, password) = credentials_str.split_at(index);
                    let username = Box::leak(username.to_owned().into_boxed_str());
                    let password = Box::leak(password[1..].to_owned().into_boxed_str());
                    return Some((username, password));
                }
            }
        }
    }
    None
}


// 将账号和密码合并为基本认证凭据的 base64 编码形式
fn encode_basic_auth_credentials(username: &str, password: &str) -> String {
    let credentials = format!("{}:{}", username, password);
    format!("{} {}", "Basic", base64::encode(credentials))
}


// 定义基本认证函数
fn response_401_body() -> String {
    let json_data = r#"{"errors":[{"code":"UNAUTHORIZED","message":"authentication required","detail":null}]}"#;
   return json_data.trim().to_string()
}
