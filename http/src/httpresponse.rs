use std::collections::HashMap;
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResonse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl Default for HttpResonse<'_> {
    fn default() -> Self {
        HttpResonse {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResonse<'a>> for String {
    fn from(res: HttpResonse) -> String {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_text(),
            res.headers(),
            res.body.clone().unwrap().len(),
            res.body(),
        )
    }
}

impl<'a> HttpResonse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut default_resp = HttpResonse::default();
        if status_code != "200" {
            default_resp.status_code = status_code;
        }
        default_resp.headers = match headers {
            Some(_) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        default_resp.status_text = match default_resp.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        default_resp.body = body;

        default_resp
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(), ()> {
        let res_string = String::from(self.clone());
        let _ = write!(write_stream, "{}", res_string);
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }
    fn status_code(&self) -> &str {
        self.status_code
    }
    fn status_text(&self) -> &str {
        self.status_text
    }
    fn headers(&self) -> String {
        // let map: HashMap<&'a str, &'a str> = self.headers.clone().unwrap();
        let mut map = HashMap::new();
        if let Some(m) = self.headers.clone() {
            map = m;
        }
        let mut header_string: String = "".to_string();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    fn body(&self) -> &str {
        let mut res = "";
        if let Some(b) = &self.body {
            res = b.as_str();
        };
        return res;
        // match self.body {
        //     Some(b) => b.as_str(),
        //     None => "",
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_struct_creation_200() {
        let resp_actual = HttpResonse::new("200", None, Some("xxxx".to_string()));
        let resp_expected = HttpResonse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut m = HashMap::new();
                m.insert("Content-Type", "text/html");
                Some(m)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(resp_actual, resp_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let resp_actual = HttpResonse::new("404", None, Some("xxxx".to_string()));
        let resp_expected = HttpResonse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut m = HashMap::new();
                m.insert("Content-Type", "text/html");
                Some(m)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(resp_actual, resp_expected);
    }

    #[test]
    fn test_response_struct_creation() {
        let resp_expected = HttpResonse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut m = HashMap::new();
                m.insert("Content-Type", "text/html");
                Some(m)
            },
            body: Some("xxxx".to_string()),
        };

        let http_string: String = resp_expected.into();
        let actual_string =
            "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nxxxx";
        assert_eq!(http_string, actual_string);
    }
}
