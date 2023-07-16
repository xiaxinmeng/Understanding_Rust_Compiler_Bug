rust
#[objc(error_enum,
    mod = ns_url_error,
    domain = unsafe { NSURLErrorDomain },
    user_info = [
        { key = NSURLErrorFailingURLErrorKey, type = Id<NSURL> },
        { key = NSURLErrorFailingURLStringErrorKey, type = Id<NSString> },
        { key = NSURLErrorBackgroundTaskCancelledReasonKey, type = ns_url_error_reasons::BackgroundTaskCancelledReason },
        { key = NSURLErrorNetworkUnavailableReasonKey, type = ns_url_error_reasons::NetworkUnavailableReason },
    ]
)]
pub enum NSURLError {
    // several variants
}
