//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[deprecated]
pub const DOM_ELEMENT_NODE: c_uint = 1;
#[deprecated]
pub const DOM_ATTRIBUTE_NODE: c_uint = 2;
#[deprecated]
pub const DOM_TEXT_NODE: c_uint = 3;
#[deprecated]
pub const DOM_CDATA_SECTION_NODE: c_uint = 4;
#[deprecated]
pub const DOM_ENTITY_REFERENCE_NODE: c_uint = 5;
#[deprecated]
pub const DOM_ENTITY_NODE: c_uint = 6;
#[deprecated]
pub const DOM_PROCESSING_INSTRUCTION_NODE: c_uint = 7;
#[deprecated]
pub const DOM_COMMENT_NODE: c_uint = 8;
#[deprecated]
pub const DOM_DOCUMENT_NODE: c_uint = 9;
#[deprecated]
pub const DOM_DOCUMENT_TYPE_NODE: c_uint = 10;
#[deprecated]
pub const DOM_DOCUMENT_FRAGMENT_NODE: c_uint = 11;
#[deprecated]
pub const DOM_NOTATION_NODE: c_uint = 12;
#[deprecated]
pub const DOM_DOCUMENT_POSITION_DISCONNECTED: c_uint = 0x01;
#[deprecated]
pub const DOM_DOCUMENT_POSITION_PRECEDING: c_uint = 0x02;
#[deprecated]
pub const DOM_DOCUMENT_POSITION_FOLLOWING: c_uint = 0x04;
#[deprecated]
pub const DOM_DOCUMENT_POSITION_CONTAINS: c_uint = 0x08;
#[deprecated]
pub const DOM_DOCUMENT_POSITION_CONTAINED_BY: c_uint = 0x10;
#[deprecated]
pub const DOM_DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: c_uint = 0x20;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMNode;

    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl ClassType for DOMNode {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMEventTarget",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMNode {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMNode {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMNode {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNode {
        #[deprecated]
        #[method_id(@__retain_semantics Other nodeName)]
        pub unsafe fn nodeName(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other nodeValue)]
        pub unsafe fn nodeValue(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setNodeValue:)]
        pub unsafe fn setNodeValue(&self, node_value: Option<&NSString>);

        #[deprecated]
        #[method(nodeType)]
        pub unsafe fn nodeType(&self) -> c_ushort;

        #[deprecated]
        #[method_id(@__retain_semantics Other parentNode)]
        pub unsafe fn parentNode(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNodeList")]
        #[deprecated]
        #[method_id(@__retain_semantics Other childNodes)]
        pub unsafe fn childNodes(&self) -> Option<Retained<DOMNodeList>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other firstChild)]
        pub unsafe fn firstChild(&self) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other lastChild)]
        pub unsafe fn lastChild(&self) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other previousSibling)]
        pub unsafe fn previousSibling(&self) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other nextSibling)]
        pub unsafe fn nextSibling(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMDocument")]
        #[deprecated]
        #[method_id(@__retain_semantics Other ownerDocument)]
        pub unsafe fn ownerDocument(&self) -> Option<Retained<DOMDocument>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other namespaceURI)]
        pub unsafe fn namespaceURI(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other prefix)]
        pub unsafe fn prefix(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setPrefix:)]
        pub unsafe fn setPrefix(&self, prefix: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other localName)]
        pub unsafe fn localName(&self) -> Retained<NSString>;

        #[cfg(feature = "DOMNamedNodeMap")]
        #[deprecated]
        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(&self) -> Option<Retained<DOMNamedNodeMap>>;

        #[method_id(@__retain_semantics Other baseURI)]
        pub unsafe fn baseURI(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other textContent)]
        pub unsafe fn textContent(&self) -> Retained<NSString>;

        #[method(setTextContent:)]
        pub unsafe fn setTextContent(&self, text_content: Option<&NSString>);

        #[cfg(feature = "DOMElement")]
        #[method_id(@__retain_semantics Other parentElement)]
        pub unsafe fn parentElement(&self) -> Option<Retained<DOMElement>>;

        #[method(isContentEditable)]
        pub unsafe fn isContentEditable(&self) -> bool;

        #[method_id(@__retain_semantics Other insertBefore:refChild:)]
        pub unsafe fn insertBefore_refChild(
            &self,
            new_child: Option<&DOMNode>,
            ref_child: Option<&DOMNode>,
        ) -> Option<Retained<DOMNode>>;

        #[method_id(@__retain_semantics Other replaceChild:oldChild:)]
        pub unsafe fn replaceChild_oldChild(
            &self,
            new_child: Option<&DOMNode>,
            old_child: Option<&DOMNode>,
        ) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other removeChild:)]
        pub unsafe fn removeChild(&self, old_child: Option<&DOMNode>) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other appendChild:)]
        pub unsafe fn appendChild(&self, new_child: Option<&DOMNode>) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method(hasChildNodes)]
        pub unsafe fn hasChildNodes(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other cloneNode:)]
        pub unsafe fn cloneNode(&self, deep: bool) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method(normalize)]
        pub unsafe fn normalize(&self);

        #[method(isSupported:version:)]
        pub unsafe fn isSupported_version(
            &self,
            feature: Option<&NSString>,
            version: Option<&NSString>,
        ) -> bool;

        #[deprecated]
        #[method(hasAttributes)]
        pub unsafe fn hasAttributes(&self) -> bool;

        #[method(isSameNode:)]
        pub unsafe fn isSameNode(&self, other: Option<&DOMNode>) -> bool;

        #[method(isEqualNode:)]
        pub unsafe fn isEqualNode(&self, other: Option<&DOMNode>) -> bool;

        #[method_id(@__retain_semantics Other lookupPrefix:)]
        pub unsafe fn lookupPrefix(
            &self,
            namespace_uri: Option<&NSString>,
        ) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other lookupNamespaceURI:)]
        pub unsafe fn lookupNamespaceURI(
            &self,
            prefix: Option<&NSString>,
        ) -> Option<Retained<NSString>>;

        #[method(isDefaultNamespace:)]
        pub unsafe fn isDefaultNamespace(&self, namespace_uri: Option<&NSString>) -> bool;

        #[method(compareDocumentPosition:)]
        pub unsafe fn compareDocumentPosition(&self, other: Option<&DOMNode>) -> c_ushort;

        #[method(contains:)]
        pub unsafe fn contains(&self, other: Option<&DOMNode>) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNode {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNode {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMNodeDeprecated
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNode {
        #[deprecated]
        #[method_id(@__retain_semantics Other insertBefore::)]
        pub unsafe fn insertBefore(
            &self,
            new_child: Option<&DOMNode>,
            ref_child: Option<&DOMNode>,
        ) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other replaceChild::)]
        pub unsafe fn replaceChild(
            &self,
            new_child: Option<&DOMNode>,
            old_child: Option<&DOMNode>,
        ) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method(isSupported::)]
        pub unsafe fn isSupported(
            &self,
            feature: Option<&NSString>,
            version: Option<&NSString>,
        ) -> bool;
    }
);