/* automatically generated by rust-bindgen 0.66.1 */

// Generated using:
// bindgen 0.66.1
// macOS SDK 13.3.

#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
use crate::dispatch_queue_t;
use core_foundation_sys::base::{Boolean, CFAllocatorRef, CFIndex, CFTypeID};
use core_foundation_sys::runloop::CFRunLoopRef;
use core_foundation_sys::string::CFStringRef;
use libc::sockaddr;
pub type __SCNetworkReachability = ::core::ffi::c_void;

pub type SCNetworkReachabilityRef = *const __SCNetworkReachability;
#[repr(C)]
pub struct SCNetworkReachabilityContext {
    pub version: CFIndex,
    pub info: *mut ::core::ffi::c_void,
    pub retain: Option<
        unsafe extern "C" fn(info: *const ::core::ffi::c_void) -> *const ::core::ffi::c_void,
    >,
    pub release: Option<unsafe extern "C" fn(info: *const ::core::ffi::c_void)>,
    pub copyDescription:
        Option<unsafe extern "C" fn(info: *const ::core::ffi::c_void) -> CFStringRef>,
}
pub type SCNetworkReachabilityFlags = u32;
pub const kSCNetworkReachabilityFlagsTransientConnection: _bindgen_ty_65 = 1;
pub const kSCNetworkReachabilityFlagsReachable: _bindgen_ty_65 = 2;
pub const kSCNetworkReachabilityFlagsConnectionRequired: _bindgen_ty_65 = 4;
pub const kSCNetworkReachabilityFlagsConnectionOnTraffic: _bindgen_ty_65 = 8;
pub const kSCNetworkReachabilityFlagsInterventionRequired: _bindgen_ty_65 = 16;
pub const kSCNetworkReachabilityFlagsConnectionOnDemand: _bindgen_ty_65 = 32;
pub const kSCNetworkReachabilityFlagsIsLocalAddress: _bindgen_ty_65 = 65536;
pub const kSCNetworkReachabilityFlagsIsDirect: _bindgen_ty_65 = 131072;
pub const kSCNetworkReachabilityFlagsIsWWAN: _bindgen_ty_65 = 262144;
pub const kSCNetworkReachabilityFlagsConnectionAutomatic: _bindgen_ty_65 = 8;
pub type _bindgen_ty_65 = ::core::ffi::c_uint;
pub type SCNetworkReachabilityCallBack = Option<
    unsafe extern "C" fn(
        target: SCNetworkReachabilityRef,
        flags: SCNetworkReachabilityFlags,
        info: *mut ::core::ffi::c_void,
    ),
>;
extern "C" {
    pub fn SCNetworkReachabilityCreateWithAddress(
        allocator: CFAllocatorRef,
        address: *const sockaddr,
    ) -> SCNetworkReachabilityRef;

    pub fn SCNetworkReachabilityCreateWithAddressPair(
        allocator: CFAllocatorRef,
        localAddress: *const sockaddr,
        remoteAddress: *const sockaddr,
    ) -> SCNetworkReachabilityRef;

    pub fn SCNetworkReachabilityCreateWithName(
        allocator: CFAllocatorRef,
        nodename: *const ::core::ffi::c_char,
    ) -> SCNetworkReachabilityRef;

    pub fn SCNetworkReachabilityGetTypeID() -> CFTypeID;

    pub fn SCNetworkReachabilityGetFlags(
        target: SCNetworkReachabilityRef,
        flags: *mut SCNetworkReachabilityFlags,
    ) -> Boolean;

    pub fn SCNetworkReachabilitySetCallback(
        target: SCNetworkReachabilityRef,
        callout: SCNetworkReachabilityCallBack,
        context: *mut SCNetworkReachabilityContext,
    ) -> Boolean;

    pub fn SCNetworkReachabilityScheduleWithRunLoop(
        target: SCNetworkReachabilityRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;

    pub fn SCNetworkReachabilityUnscheduleFromRunLoop(
        target: SCNetworkReachabilityRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;

    pub fn SCNetworkReachabilitySetDispatchQueue(
        target: SCNetworkReachabilityRef,
        queue: dispatch_queue_t,
    ) -> Boolean;
}