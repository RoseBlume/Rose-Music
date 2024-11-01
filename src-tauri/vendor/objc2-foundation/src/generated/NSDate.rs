//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSSystemClockDidChangeNotification: &'static NSNotificationName;
}

pub type NSTimeInterval = c_double;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDate;

    unsafe impl ClassType for NSDate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSDate {}

unsafe impl Sync for NSDate {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSDate {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSDate {}

unsafe impl NSObjectProtocol for NSDate {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSDate {}

extern_methods!(
    unsafe impl NSDate {
        #[method(timeIntervalSinceReferenceDate)]
        pub unsafe fn timeIntervalSinceReferenceDate(&self) -> NSTimeInterval;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithTimeIntervalSinceReferenceDate:)]
        pub unsafe fn initWithTimeIntervalSinceReferenceDate(
            this: Allocated<Self>,
            ti: NSTimeInterval,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDate {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSExtendedDate
    unsafe impl NSDate {
        #[method(timeIntervalSinceDate:)]
        pub unsafe fn timeIntervalSinceDate(&self, another_date: &NSDate) -> NSTimeInterval;

        #[method(timeIntervalSinceNow)]
        pub unsafe fn timeIntervalSinceNow(&self) -> NSTimeInterval;

        #[method(timeIntervalSince1970)]
        pub unsafe fn timeIntervalSince1970(&self) -> NSTimeInterval;

        #[deprecated = "Use dateByAddingTimeInterval instead"]
        #[method_id(@__retain_semantics Other addTimeInterval:)]
        pub unsafe fn addTimeInterval(&self, seconds: NSTimeInterval) -> Retained<AnyObject>;

        #[method_id(@__retain_semantics Other dateByAddingTimeInterval:)]
        pub unsafe fn dateByAddingTimeInterval(&self, ti: NSTimeInterval) -> Retained<Self>;

        #[method_id(@__retain_semantics Other earlierDate:)]
        pub unsafe fn earlierDate(&self, another_date: &NSDate) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other laterDate:)]
        pub unsafe fn laterDate(&self, another_date: &NSDate) -> Retained<NSDate>;

        #[cfg(feature = "NSObjCRuntime")]
        #[method(compare:)]
        pub unsafe fn compare(&self, other: &NSDate) -> NSComparisonResult;

        #[method(isEqualToDate:)]
        pub unsafe fn isEqualToDate(&self, other_date: &NSDate) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(
            &self,
            locale: Option<&AnyObject>,
        ) -> Retained<NSString>;

        #[method(timeIntervalSinceReferenceDate)]
        pub unsafe fn timeIntervalSinceReferenceDate_class() -> NSTimeInterval;
    }
);

extern_methods!(
    /// NSDateCreation
    unsafe impl NSDate {
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date() -> Retained<Self>;

        #[method_id(@__retain_semantics Other dateWithTimeIntervalSinceNow:)]
        pub unsafe fn dateWithTimeIntervalSinceNow(secs: NSTimeInterval) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dateWithTimeIntervalSinceReferenceDate:)]
        pub unsafe fn dateWithTimeIntervalSinceReferenceDate(ti: NSTimeInterval) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dateWithTimeIntervalSince1970:)]
        pub unsafe fn dateWithTimeIntervalSince1970(secs: NSTimeInterval) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dateWithTimeInterval:sinceDate:)]
        pub unsafe fn dateWithTimeInterval_sinceDate(
            secs_to_be_added: NSTimeInterval,
            date: &NSDate,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other distantFuture)]
        pub unsafe fn distantFuture() -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other distantPast)]
        pub unsafe fn distantPast() -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other now)]
        pub unsafe fn now() -> Retained<NSDate>;

        #[method_id(@__retain_semantics Init initWithTimeIntervalSinceNow:)]
        pub unsafe fn initWithTimeIntervalSinceNow(
            this: Allocated<Self>,
            secs: NSTimeInterval,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithTimeIntervalSince1970:)]
        pub unsafe fn initWithTimeIntervalSince1970(
            this: Allocated<Self>,
            secs: NSTimeInterval,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithTimeInterval:sinceDate:)]
        pub unsafe fn initWithTimeInterval_sinceDate(
            this: Allocated<Self>,
            secs_to_be_added: NSTimeInterval,
            date: &NSDate,
        ) -> Retained<Self>;
    }
);