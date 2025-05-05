use std::{fmt::{Debug, Display}, panic::Location};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Message(Message),
    Error(TraceError),
}
impl Error {
    fn _fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = f.debug_tuple("Error");
        match self {
            Self::Message(msg) => {
                r.field(msg);
            },
            Self::Error(err) => {
                r.field(err);
            }
        }
        r.finish()
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self._fmt(f)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self._fmt(f)
    }
}

pub struct Message {
    pub code: Option<String>,
    pub message: String,
}
impl Message {
    fn _fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = f.debug_struct("Message");
        if let Some(v) = &self.code {
            r.field("code", v);
        }
        r.field("message", &self.message);
        r.finish()
    }
}
impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self._fmt(f)
    }
}
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self._fmt(f)
    }
}


pub struct TraceError {
    pub code: Option<String>,
    pub info: Option<String>,
    pub error: Option<Box<dyn Debug>>,
    pub trace: Option<String>,
    pub ext: Option<String>,
}
impl TraceError {
    fn _fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = f.debug_struct("TraceError");
        if let Some(v) = &self.code {
            r.field("code", v);
        }
        if let Some(v) = &self.info {
            r.field("info", v);
        }
        if let Some(v) = &self.error {
            r.field("error", v);
        }
        if let Some(v) = &self.trace {
            r.field("trace", v);
        }
        if let Some(v) = &self.ext {
            r.field("ext", v);
        }
        r.finish()
    }
}
impl Debug for TraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self._fmt(f)
    }
}
impl Display for TraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self._fmt(f)
    }
}

impl Error {
    
    pub fn message(message: &str) -> Self {
        Self::Message(Message { code: None, message: message.to_string() })
    }
    pub fn code_message(code: &str, message: &str) -> Self {
        Self::Message(Message { code: Some(code.to_string()), message: message.to_string() })
    }

    #[track_caller]
    pub fn error(error: Box<dyn Debug>) -> Self {
        let caller = Location::caller();
        let file_path = _deal_with_file_path(caller.file());
        let trace = format!("{}:{},{}", file_path, caller.line(), caller.column());
        Self::Error(TraceError { code: None, info: None, error: Some(error), trace: Some(trace), ext: None })
    }

    #[track_caller]
    pub fn error_info(info: &str) -> Self {
        let caller = Location::caller();
        let file_path = _deal_with_file_path(caller.file());
        let trace = format!("{}:{},{}", file_path, caller.line(), caller.column());
        Self::Error(TraceError { code: None, info: Some(info.to_string()), error: None, trace: Some(trace), ext: None })
    }

    #[track_caller]
    pub fn error_code_info(code: &str, info: &str) -> Self {
        let caller = Location::caller();
        let file_path = _deal_with_file_path(caller.file());
        let trace = format!("{}:{},{}", file_path, caller.line(), caller.column());
        Self::Error(TraceError { code: Some(code.to_string()), info: Some(info.to_string()), error: None, trace: Some(trace), ext: None })
    }

    pub fn ext(self, ext: &str) -> Self {
        match self {
            Self::Message(_) => self,
            Self::Error(mut err) => {
                err.ext = Some(ext.to_string());
                Self::Error(err)
            }
        }
    }

}

fn _deal_with_file_path<'a>(path: &'a str) -> &'a str {
    if let Some(s) = path.split_once("src/") {
        s.1
    } else {
        ""
    }
}

