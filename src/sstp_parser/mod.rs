use crate::sstp_request::SstpRequest;

pub struct SstpParser;

impl SstpParser {
    pub fn parse(req: &str) -> SstpRequest {
        let lines: Vec<&str> = req.split("\r\n").collect();
        let mut ret = SstpRequest::new();

        lines.iter().enumerate().for_each(|(index, line)| {
            if line.is_empty() {
                return;
            }

            let mut kv = if index == 0 {
                line.split(" ")
            } else {
                line.split(": ")
            };

            let key = kv.next().unwrap();
            let value = kv.next().unwrap();

            if index == 0 {
                ret.set_header("method", key);
                ret.set_header("version", value);
            } else {
                ret.set_header(key, value);
            }
        });

        ret
    }
}
