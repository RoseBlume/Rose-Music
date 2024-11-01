//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[deprecated]
pub const DOM_CSS_UNKNOWN: c_uint = 0;
#[deprecated]
pub const DOM_CSS_NUMBER: c_uint = 1;
#[deprecated]
pub const DOM_CSS_PERCENTAGE: c_uint = 2;
#[deprecated]
pub const DOM_CSS_EMS: c_uint = 3;
#[deprecated]
pub const DOM_CSS_EXS: c_uint = 4;
#[deprecated]
pub const DOM_CSS_PX: c_uint = 5;
#[deprecated]
pub const DOM_CSS_CM: c_uint = 6;
#[deprecated]
pub const DOM_CSS_MM: c_uint = 7;
#[deprecated]
pub const DOM_CSS_IN: c_uint = 8;
#[deprecated]
pub const DOM_CSS_PT: c_uint = 9;
#[deprecated]
pub const DOM_CSS_PC: c_uint = 10;
#[deprecated]
pub const DOM_CSS_DEG: c_uint = 11;
#[deprecated]
pub const DOM_CSS_RAD: c_uint = 12;
#[deprecated]
pub const DOM_CSS_GRAD: c_uint = 13;
#[deprecated]
pub const DOM_CSS_MS: c_uint = 14;
#[deprecated]
pub const DOM_CSS_S: c_uint = 15;
#[deprecated]
pub const DOM_CSS_HZ: c_uint = 16;
#[deprecated]
pub const DOM_CSS_KHZ: c_uint = 17;
#[deprecated]
pub const DOM_CSS_DIMENSION: c_uint = 18;
#[deprecated]
pub const DOM_CSS_STRING: c_uint = 19;
#[deprecated]
pub const DOM_CSS_URI: c_uint = 20;
#[deprecated]
pub const DOM_CSS_IDENT: c_uint = 21;
#[deprecated]
pub const DOM_CSS_ATTR: c_uint = 22;
#[deprecated]
pub const DOM_CSS_COUNTER: c_uint = 23;
#[deprecated]
pub const DOM_CSS_RECT: c_uint = 24;
#[deprecated]
pub const DOM_CSS_RGBCOLOR: c_uint = 25;
#[deprecated]
pub const DOM_CSS_VW: c_uint = 26;
#[deprecated]
pub const DOM_CSS_VH: c_uint = 27;
#[deprecated]
pub const DOM_CSS_VMIN: c_uint = 28;
#[deprecated]
pub const DOM_CSS_VMAX: c_uint = 29;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMCSSValue",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMCSSPrimitiveValue;

    #[cfg(all(
        feature = "DOMCSSValue",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMCSSPrimitiveValue {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCSSValue;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMCSSValue",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMCSSPrimitiveValue {}

#[cfg(all(
    feature = "DOMCSSValue",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMCSSPrimitiveValue {}

extern_methods!(
    #[cfg(all(
        feature = "DOMCSSValue",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSPrimitiveValue {
        #[deprecated]
        #[method(primitiveType)]
        pub unsafe fn primitiveType(&self) -> c_ushort;

        #[method(setFloatValue:floatValue:)]
        pub unsafe fn setFloatValue_floatValue(&self, unit_type: c_ushort, float_value: c_float);

        #[deprecated]
        #[method(getFloatValue:)]
        pub unsafe fn getFloatValue(&self, unit_type: c_ushort) -> c_float;

        #[method(setStringValue:stringValue:)]
        pub unsafe fn setStringValue_stringValue(
            &self,
            string_type: c_ushort,
            string_value: Option<&NSString>,
        );

        #[deprecated]
        #[method_id(@__retain_semantics Other getStringValue)]
        pub unsafe fn getStringValue(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "DOMCounter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other getCounterValue)]
        pub unsafe fn getCounterValue(&self) -> Option<Retained<DOMCounter>>;

        #[cfg(feature = "DOMRect")]
        #[deprecated]
        #[method_id(@__retain_semantics Other getRectValue)]
        pub unsafe fn getRectValue(&self) -> Option<Retained<DOMRect>>;

        #[cfg(feature = "DOMRGBColor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other getRGBColorValue)]
        pub unsafe fn getRGBColorValue(&self) -> Option<Retained<DOMRGBColor>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMCSSValue",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSPrimitiveValue {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMCSSValue",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSPrimitiveValue {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMCSSPrimitiveValueDeprecated
    #[cfg(all(
        feature = "DOMCSSValue",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSPrimitiveValue {
        #[deprecated]
        #[method(setFloatValue::)]
        pub unsafe fn setFloatValue(&self, unit_type: c_ushort, float_value: c_float);

        #[deprecated]
        #[method(setStringValue::)]
        pub unsafe fn setStringValue(&self, string_type: c_ushort, string_value: Option<&NSString>);
    }
);