pub enum StatusCode {
    Ok,
    NoContent,
    BadRequest,
    Unauthorized,
    NotFound,
    InternalServerError,
    NA,
}

impl From<&StatusCode> for i32 {
    fn from(status: &StatusCode) -> Self {
        match status {
            StatusCode::Ok => 200,
            StatusCode::NoContent => 204,
            StatusCode::BadRequest => 400,
            StatusCode::Unauthorized => 401,
            StatusCode::NotFound => 404,
            StatusCode::InternalServerError => 500,
            StatusCode::NA => 0,
        }
    }
}

impl From<&StatusCode> for String {
    fn from(status: &StatusCode) -> Self {
        match status {
            StatusCode::Ok => "OK".into(),
            StatusCode::NoContent => "NoContent".into(),
            StatusCode::BadRequest => "BadRequest".into(),
            StatusCode::Unauthorized => "Unauthorized".into(),
            StatusCode::NotFound => "NotFound".into(),
            StatusCode::InternalServerError => "InternalServerError".into(),
            StatusCode::NA => "NA".into(),
        }
    }
}

pub struct SstpResponse {
    status: StatusCode,
    headers: Vec<(String, String)>,
}

impl SstpResponse {
    pub fn new() -> Self {
        Self {
            status: StatusCode::NA,
            headers: Vec::new(),
        }
    }

    pub fn set_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn set_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    pub fn to_string(&self) -> String {
        let mut ret = format!(
            "PLUGIN/2.0 {} {}\r\n",
            i32::from(&self.status),
            String::from(&self.status)
        );
        self.headers.iter().for_each(|(k, v)| {
            ret.push_str(&format!("{}: {}\r\n", k, v));
        });
        ret.push_str("\r\n\r\n");
        ret
    }
}
