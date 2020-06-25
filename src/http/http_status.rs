#[derive(Debug, Eq, PartialEq)]
pub struct HttpStatus {
    pub code: u16,
    pub desc: &'static str,
}

// predefined http response status
impl HttpStatus {
    // TODO: declare remaining http status codes
    // https://developer.mozilla.org/ko/docs/Web/HTTP/Status
    // Informational
    pub const CONTINUE: HttpStatus = create_http_status(100, "Continue");
    pub const SWITCHING_PROTOCOL: HttpStatus = create_http_status(101, "Switching Protocol");
    // 102
    // 103

    // Success
    pub const OK: HttpStatus = create_http_status(200, "OK");
    pub const CREATED: HttpStatus = create_http_status(201, "Created");
    pub const ACCEPTED: HttpStatus = create_http_status(202, "Accepted");
    // 203
    // 204
    // 205
    // 206
    // 207
    // 208
    // 226

    // Redirection
    // 300
    pub const MOVED_PERMANENTLY: HttpStatus = create_http_status(301, "Moved Permanently");
    pub const FOUND: HttpStatus = create_http_status(302, "Found");
    // 303
    // 304
    // 305
    // 306
    // 307
    // 308

    // Client Error
    pub const BAD_REQUEST: HttpStatus = create_http_status(400, "Bad Request");
    pub const UNAUTHORIZED: HttpStatus = create_http_status(401, "Unauthorized");
    // 402
    // 403
    // 404
    pub const NOT_FOUND: HttpStatus = create_http_status(404, "Not Found");
    // 405
    // 406
    // 407
    // 408
    // 409
    // 410
    // 411
    // 412
    // 413
    // 414
    // 415
    // 416
    // 417
    // 418
    // 421
    // 422
    // 423
    // 424
    // 426
    // 428
    // 429
    // 431
    // 451

    // Server Error
    pub const INTERNAL_SERVER_ERROR: HttpStatus = create_http_status(500, "Internal Server Error");
    pub const NOT_IMPLEMENTED: HttpStatus = create_http_status(501, "Not Implemented");
    // 502
    // 503
    // 504
    // 505
    // 506
    // 507
    // 508
    // 510
    // 511

    // can create undefined HttpStatus
    pub fn unknown(code: u16, desc: &'static str) -> HttpStatus {
        HttpStatus { code, desc }
    }
}

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        format!("{} {}", self.code, self.desc)
    }
}

const fn create_http_status(code: u16, desc: &'static str) -> HttpStatus {
    HttpStatus { code, desc }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant() {
        let status_1 = HttpStatus::OK;
        let status_2 = HttpStatus::OK;

        assert_eq!(status_1.code, 200);
        assert_eq!(status_1.desc, "OK");
        assert_eq!(status_1, status_2);
    }

    fn get_status(code: u16) -> HttpStatus {
        if code == 200 {
            HttpStatus::OK
        } else {
            HttpStatus::unknown(code, "unknown")
        }
    }

    #[test]
    fn test_match() {
        let ok = get_status(200);
        assert_eq!(ok, HttpStatus::OK);

        let unknown = get_status(1);
        assert_eq!(unknown.code, 1);
    }
}
