//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub type NSHelpBookName = NSString;

pub type NSHelpAnchorName = NSString;

pub type NSHelpManagerContextHelpKey = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHelpManager;

    unsafe impl ClassType for NSHelpManager {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for NSHelpManager {}

extern_methods!(
    unsafe impl NSHelpManager {
        #[method_id(@__retain_semantics Other sharedHelpManager)]
        pub unsafe fn sharedHelpManager(mtm: MainThreadMarker) -> Retained<NSHelpManager>;

        #[method(isContextHelpModeActive)]
        pub unsafe fn isContextHelpModeActive(mtm: MainThreadMarker) -> bool;

        #[method(setContextHelpModeActive:)]
        pub unsafe fn setContextHelpModeActive(
            context_help_mode_active: bool,
            mtm: MainThreadMarker,
        );

        #[method(setContextHelp:forObject:)]
        pub unsafe fn setContextHelp_forObject(
            &self,
            attr_string: &NSAttributedString,
            object: &AnyObject,
        );

        #[method(removeContextHelpForObject:)]
        pub unsafe fn removeContextHelpForObject(&self, object: &AnyObject);

        #[method_id(@__retain_semantics Other contextHelpForObject:)]
        pub unsafe fn contextHelpForObject(
            &self,
            object: &AnyObject,
        ) -> Option<Retained<NSAttributedString>>;

        #[method(showContextHelpForObject:locationHint:)]
        pub unsafe fn showContextHelpForObject_locationHint(
            &self,
            object: &AnyObject,
            pt: NSPoint,
        ) -> bool;

        #[method(openHelpAnchor:inBook:)]
        pub unsafe fn openHelpAnchor_inBook(
            &self,
            anchor: &NSHelpAnchorName,
            book: Option<&NSHelpBookName>,
        );

        #[method(findString:inBook:)]
        pub unsafe fn findString_inBook(&self, query: &NSString, book: Option<&NSHelpBookName>);

        #[method(registerBooksInBundle:)]
        pub unsafe fn registerBooksInBundle(&self, bundle: &NSBundle) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSHelpManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    pub static NSContextHelpModeDidActivateNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSContextHelpModeDidDeactivateNotification: &'static NSNotificationName;
}

extern_category!(
    /// Category on [`NSBundle`].
    pub unsafe trait NSBundleHelpExtension {
        #[method_id(@__retain_semantics Other contextHelpForKey:)]
        unsafe fn contextHelpForKey(
            &self,
            key: &NSHelpManagerContextHelpKey,
        ) -> Option<Retained<NSAttributedString>>;
    }

    unsafe impl NSBundleHelpExtension for NSBundle {}
);

extern_methods!(
    /// NSApplicationHelpExtension
    #[cfg(all(feature = "NSApplication", feature = "NSResponder"))]
    unsafe impl NSApplication {
        #[method(activateContextHelpMode:)]
        pub unsafe fn activateContextHelpMode(&self, sender: Option<&AnyObject>);

        #[method(showHelp:)]
        pub unsafe fn showHelp(&self, sender: Option<&AnyObject>);
    }
);