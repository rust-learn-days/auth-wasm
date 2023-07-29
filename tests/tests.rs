// 模块添加 #[cfg(test)] 属性，表示这些代码仅在测试环境下编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_basic_auth_credentials() {
        // 测试合法的Basic认证凭据
        let valid_auth_header = "Basic dXNlcjpwd2Q=";
        assert_eq!(
            extract_basic_auth_credentials(valid_auth_header),
            Some(("user", "pwd"))
        );

        // 测试无效的Basic认证凭据
        let invalid_auth_header = "Bearer abcdefg";
        assert_eq!(extract_basic_auth_credentials(invalid_auth_header), None);

        // 测试空的Authorization字段
        let empty_auth_header = "";
        assert_eq!(extract_basic_auth_credentials(empty_auth_header), None);
    }

    #[test]
    fn test_encode_basic_auth_credentials() {
        // 测试编码账号和密码为Basic认证凭据
        let username = "admin";
        let password = "passw0rd";
        assert_eq!(
            encode_basic_auth_credentials(username, password),
            "YWRtaW46cGFzc3cwcmQ="
        );
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
    base64::encode(credentials)
}
