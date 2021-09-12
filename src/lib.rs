#[cfg(feature="nsurlsession")]
mod nsurlsession;
#[cfg(feature="nsurlrequest")]
mod nsurlrequest;
#[cfg(feature="nsurl")]
mod nsurl;
#[cfg(feature="nsdata")]
mod nsdata;
mod types;
#[cfg(feature="nsurlresponse")]
mod nsurlresponse;
mod nsstring;
pub mod magic_string;
#[cfg(feature="nsthread")]
mod nsthread;
#[cfg(feature="nsdictionary")]
mod nsdictionary;
#[cfg(feature="nscopying")]
mod nscopying;
#[cfg(feature="nsvalue")]
mod nsvalue;
#[cfg(feature="nsnotification")]
mod nsnotification;

pub use objr::foundation::*;
pub use types::{NSUInteger,NSInteger};
pub use nsstring::NSStringExtension;

#[cfg(feature="nsurl")]
pub use nsurl::NSURL;
#[cfg(feature="nsdata")]
pub use nsdata::NSData;
#[cfg(feature="nsurlresponse")]
pub use nsurlresponse::NSURLResponse;
#[cfg(feature="nsurlrequest")]
pub use nsurlrequest::NSURLRequest;
#[cfg(feature="nsurlrequest")]
pub use nsurlrequest::NSMutableURLRequest;
#[cfg(feature="nsurlsession")]
pub use nsurlsession::{NSURLSession,NSURLSessionDownloadTask,NSURLSessionDataTask};
#[cfg(feature="nsthread")]
pub use nsthread::NSThread;
#[cfg(feature="nsdictionary")]
pub use nsdictionary::{NSDictionary,NSDictionaryRaw};
#[cfg(feature="nscopying")]
pub use nscopying::NSCopying;
#[cfg(feature="nsvalue")]
pub use nsvalue::NSNumber;
#[cfg(feature="nsnotification")]
pub use nsnotification::{NSNotification,NSNotificationName};