use std::io::{BufReader, Cursor};

use rvvc::VoiceVoxCore;

use crate::{
    sstp_response::{SstpResponse, StatusCode},
    write_log,
};

use super::sstp_parser::SstpParser;

pub fn main(req: &str) -> String {
    let req = SstpParser::parse(req);

    let method = req.method().unwrap_or_default();
    let id = req.id().unwrap_or_default();
    let log_msg = format!("method [{}], id [{}]\r\n", method, id);
    if id != "OnSecondChange" {
        write_log(log_msg.as_bytes());
    }

    match id.as_str() {
        "version" => {
            write_log("# version\r\n".as_bytes());
            write_log(req.to_string().as_bytes());

            SstpResponse::new()
                .set_status(StatusCode::Ok)
                .set_header("Charset", "UTF-8")
                .set_header("Value", "DummyPlug")
                .to_string()
        }
        "OnMenuExec" => {
            write_log("# OnMenuExec\r\n".as_bytes());
            write_log(req.to_string().as_bytes());

            SstpResponse::new()
                .set_status(StatusCode::Ok)
                .set_header("Charset", "UTF-8")
                .set_header("SecurityLevel", "local")
                .set_header("Marker", "Dummy Plug Menu")
                .set_header("Script", "\\h\\f[bold,true][機能選択]\\f[bold,normal]\\n[half]\\n\\![*]\\q[テスト,test]\\n\\![*]\\q[キャンセル,cancel]\\e")
                .set_header("Event", "OnDummyPlugEvent")
                .set_header("Reference0", "テスト,test")
                .set_header("Reference1", "キャンセル,cancel")
                .to_string()
        }
        "OnChoiceSelectEx" => {
            write_log("# OnChoiceSelectEx\r\n".as_bytes());
            write_log(req.to_string().as_bytes());
            let r0 = req.get_header("Reference0").unwrap_or_default();
            let r1 = req.get_header("Reference1").unwrap_or_default();
            let r2 = req.get_header("Reference2").unwrap_or_default();
            write_log(format!("Reference [{}], [{}], [{}]\r\n", r0, r1, r2).as_bytes());

            SstpResponse::new()
                .set_status(StatusCode::NoContent)
                .set_header("Charset", "UTF-8")
                .to_string()
        }
        "OnOtherGhostTalk" => {
            write_log("# OnOtherGhostTalk\r\n".as_bytes());
            write_log(req.to_string().as_bytes());
            let result = crate::audio_pool::enqueue(move || {
                let mut core = VoiceVoxCore::new();

                write_log(b"create query\r\n");
                let text = req.get_header("Reference4").unwrap_or_default();
                write_log(format!("before {}\r\n", text).as_bytes());
                let text = regex::Regex::new(r"\\_*[!?\-+*]*[a-zA-Z0-9]*(\[.*?\])*")
                    .unwrap()
                    .replace_all(&text, "")
                    .to_string();
                if text.is_empty() {
                    write_log(b"empty text\r\n");
                    return;
                };
                write_log(format!("after {}\r\n", text).as_bytes());
                let query = match core.audio_query(&text, 1) {
                    Ok(query) => query,
                    Err(e) => {
                        write_log(format!("failed to audio_query {}\r\n", e).as_bytes());
                        return;
                    }
                };

                write_log(b"create wav\r\n");
                let wav = match core.synthesis(query) {
                    Ok(wav) => wav,
                    Err(e) => {
                        write_log(format!("failed to synthesis {}\r\n", e).as_bytes());
                        return;
                    }
                };

                use rodio::OutputStream;

                write_log(b"create stream\r\n");
                let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
                    write_log(b"failed to get stream\r\n");
                    return;
                };

                write_log(b"play wav...\r\n");
                let sink = match stream_handle.play_once(BufReader::new(Cursor::new(wav))) {
                    Ok(sink) => sink,
                    Err(e) => {
                        write_log(format!("failed to play {}\r\n", e).as_bytes());
                        return;
                    }
                };

                sink.sleep_until_end();

                write_log(b"play wav...done\r\n");
            });

            if let Err(e) = result {
                write_log(format!("failed to enqueue {}\r\n", e).as_bytes());
            }

            write_log(b"done\r\n");
            "PLUGIN/2.0 204 NoContent\r\n\r\n".to_string()
        }
        _ => "PLUGIN/2.0 204 NoContent\r\n\r\n".to_string(),
    }
}
