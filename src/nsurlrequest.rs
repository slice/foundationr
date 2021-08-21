use objr::bindings::*;
use super::{NSURL,NSData};
objc_class! {
    pub struct NSURLRequest {
        @class(NSURLRequest)
    }
}
objc_selector_group! {
    trait NSURLRequestSelectors {
        @selector("initWithURL:")
        @selector("setValue:forHTTPHeaderField:")
        @selector("setHTTPMethod:")
        @selector("setHTTPBody:")
    }
    impl NSURLRequestSelectors for Sel {}
}


objc_class! {
    pub struct NSMutableURLRequest {
        @class(NSMutableURLRequest)
    }
}
#[allow(non_snake_case)]
impl NSMutableURLRequest {
    pub fn as_immutable(&self) -> &NSURLRequest {
        unsafe{ self.cast() }
    }
    pub fn with_url(url: &NSURL, pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe{
            let uninitialized = NSMutableURLRequest::class().alloc(pool);
            Self::assume_nonnil(Self::perform(uninitialized, Sel::initWithURL_(), pool, (url,))).assume_retained().assume_mut()
        }
    }
    pub fn setValueForHTTPHeaderField(&mut self,value:Option<&NSString>,header_field: &NSString, pool: &ActiveAutoreleasePool) {
        unsafe{
            Self::perform_primitive(self, Sel::setValue_forHTTPHeaderField(), pool, (value.as_ptr(), header_field))
        }
    }
    pub fn setHTTPMethod(&mut self, value: &NSString, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setHTTPMethod_(), pool, (value,))
        }
    }
    pub fn setHTTPBody(&mut self, value: &NSData, pool: &ActiveAutoreleasePool) {
        unsafe{
            Self::perform_primitive(self, Sel::setHTTPBody_(), pool, (value,))
        }
    }

}

#[test] fn with_url() {
    let pool = AutoreleasePool::new();
    let mut request = NSMutableURLRequest::with_url(&NSURL::from_string(objc_nsstring!("https://sealedabstract.com"),&pool).unwrap(),&pool);
    request.setValueForHTTPHeaderField(Some(objc_nsstring!("value")),objc_nsstring!("header"),&pool);
    request.setHTTPMethod(objc_nsstring!("POST"),&pool);
    let body_strong = "My body";
    unsafe{
        let body_data = NSData::from_borrowed_bytes(body_strong.as_bytes(),&pool);
        request.setHTTPBody(&body_data,&pool);
    }

    println!("{}",request);
}