//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMCharacterData",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMText;

    #[cfg(all(
        feature = "DOMCharacterData",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMText {
        #[inherits(DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCharacterData;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMCharacterData",
    feature = "DOMEventTarget",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMText {}

#[cfg(all(
    feature = "DOMCharacterData",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMText {}

#[cfg(all(
    feature = "DOMCharacterData",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMText {}

extern_methods!(
    #[cfg(all(
        feature = "DOMCharacterData",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMText {
        #[method_id(@__retain_semantics Other wholeText)]
        pub unsafe fn wholeText(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other splitText:)]
        pub unsafe fn splitText(&self, offset: c_uint) -> Option<Retained<DOMText>>;

        #[method_id(@__retain_semantics Other replaceWholeText:)]
        pub unsafe fn replaceWholeText(
            &self,
            content: Option<&NSString>,
        ) -> Option<Retained<DOMText>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMCharacterData",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMText {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMCharacterData",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMText {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);