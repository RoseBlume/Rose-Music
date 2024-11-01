//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAEDRMetadata;

    unsafe impl ClassType for CAEDRMetadata {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CAEDRMetadata {}

unsafe impl NSCopying for CAEDRMetadata {}

unsafe impl NSObjectProtocol for CAEDRMetadata {}

unsafe impl NSSecureCoding for CAEDRMetadata {}

extern_methods!(
    unsafe impl CAEDRMetadata {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other HDR10MetadataWithDisplayInfo:contentInfo:opticalOutputScale:)]
        pub unsafe fn HDR10MetadataWithDisplayInfo_contentInfo_opticalOutputScale(
            display_data: Option<&NSData>,
            content_data: Option<&NSData>,
            scale: c_float,
        ) -> Retained<CAEDRMetadata>;

        #[method_id(@__retain_semantics Other HDR10MetadataWithMinLuminance:maxLuminance:opticalOutputScale:)]
        pub unsafe fn HDR10MetadataWithMinLuminance_maxLuminance_opticalOutputScale(
            min_nits: c_float,
            max_nits: c_float,
            scale: c_float,
        ) -> Retained<CAEDRMetadata>;

        #[method_id(@__retain_semantics Other HLGMetadataWithAmbientViewingEnvironment:)]
        pub unsafe fn HLGMetadataWithAmbientViewingEnvironment(
            data: &NSData,
        ) -> Retained<CAEDRMetadata>;

        #[method_id(@__retain_semantics Other HLGMetadata)]
        pub unsafe fn HLGMetadata() -> Retained<CAEDRMetadata>;

        #[method(isAvailable)]
        pub unsafe fn isAvailable() -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAEDRMetadata {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);