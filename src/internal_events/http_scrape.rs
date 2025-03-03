use metrics::counter;
use vector_common::internal_event::{error_stage, error_type};
use vector_core::internal_event::InternalEvent;

use super::prelude::http_error_code;

#[derive(Debug)]
pub struct HttpScrapeEventsReceived {
    pub byte_size: usize,
    pub count: usize,
    pub url: String,
}

impl InternalEvent for HttpScrapeEventsReceived {
    fn emit(self) {
        trace!(
            message = "Events received.",
            count = %self.count,
            byte_size = %self.byte_size,
            url = %self.url,
        );
        counter!(
            "component_received_events_total", self.count as u64,
            "uri" => self.url.clone(),
        );
        counter!(
            "component_received_event_bytes_total", self.byte_size as u64,
            "uri" => self.url.clone(),
        );
        // deprecated
        counter!(
            "events_in_total", self.count as u64,
            "uri" => self.url,
        );
    }
}

#[derive(Debug)]
pub struct HttpScrapeHttpResponseError {
    pub code: hyper::StatusCode,
    pub url: String,
}

impl InternalEvent for HttpScrapeHttpResponseError {
    fn emit(self) {
        error!(
            message = "HTTP error response.",
            url = %self.url,
            stage = error_stage::RECEIVING,
            error_type = error_type::REQUEST_FAILED,
            error_code = %http_error_code(self.code.as_u16()),
            internal_log_rate_secs = 10,
        );
        counter!(
            "component_errors_total", 1,
            "url" => self.url,
            "stage" => error_stage::RECEIVING,
            "error_type" => error_type::REQUEST_FAILED,
            "error_code" => http_error_code(self.code.as_u16()),
        );
        // deprecated
        counter!("http_error_response_total", 1);
    }
}

#[derive(Debug)]
pub struct HttpScrapeHttpError {
    pub error: crate::Error,
    pub url: String,
}

impl InternalEvent for HttpScrapeHttpError {
    fn emit(self) {
        error!(
            message = "HTTP request processing error.",
            url = %self.url,
            error = ?self.error,
            error_type = error_type::REQUEST_FAILED,
            stage = error_stage::RECEIVING,
            internal_log_rate_secs = 10,
        );
        counter!(
            "component_errors_total", 1,
            "url" => self.url,
            "error_type" => error_type::REQUEST_FAILED,
            "stage" => error_stage::RECEIVING,
        );
        // deprecated
        counter!("http_request_errors_total", 1);
    }
}
