//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSSharingServiceName = NSString;

extern "C" {
    pub static NSSharingServiceNameComposeEmail: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNameComposeMessage: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNameSendViaAirDrop: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNameAddToSafariReadingList: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNameAddToIPhoto: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNameAddToAperture: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNameUseAsDesktopPicture: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNamePostOnFacebook: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNamePostOnTwitter: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNamePostOnSinaWeibo: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNamePostOnTencentWeibo: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNamePostOnLinkedIn: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNameUseAsTwitterProfileImage: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNameUseAsFacebookProfileImage: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNameUseAsLinkedInProfileImage: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNamePostImageOnFlickr: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNamePostVideoOnVimeo: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNamePostVideoOnYouku: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNamePostVideoOnTudou: &'static NSSharingServiceName;
}

extern "C" {
    pub static NSSharingServiceNameCloudSharing: &'static NSSharingServiceName;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSharingService;

    unsafe impl ClassType for NSSharingService {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSSharingService {}

extern_methods!(
    unsafe impl NSSharingService {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<ProtocolObject<dyn NSSharingServiceDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSharingServiceDelegate>>,
        );

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Retained<NSImage>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Retained<NSImage>>;

        #[method_id(@__retain_semantics Other menuItemTitle)]
        pub unsafe fn menuItemTitle(&self) -> Retained<NSString>;

        #[method(setMenuItemTitle:)]
        pub unsafe fn setMenuItemTitle(&self, menu_item_title: &NSString);

        #[method_id(@__retain_semantics Other recipients)]
        pub unsafe fn recipients(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(setRecipients:)]
        pub unsafe fn setRecipients(&self, recipients: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other subject)]
        pub unsafe fn subject(&self) -> Option<Retained<NSString>>;

        #[method(setSubject:)]
        pub unsafe fn setSubject(&self, subject: Option<&NSString>);

        #[method_id(@__retain_semantics Other messageBody)]
        pub unsafe fn messageBody(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other permanentLink)]
        pub unsafe fn permanentLink(&self) -> Option<Retained<NSURL>>;

        #[method_id(@__retain_semantics Other accountName)]
        pub unsafe fn accountName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other attachmentFileURLs)]
        pub unsafe fn attachmentFileURLs(&self) -> Option<Retained<NSArray<NSURL>>>;

        #[deprecated = "Use -[NSSharingServicePicker standardShareMenuItem] instead."]
        #[method_id(@__retain_semantics Other sharingServicesForItems:)]
        pub unsafe fn sharingServicesForItems(
            items: &NSArray,
        ) -> Retained<NSArray<NSSharingService>>;

        #[method_id(@__retain_semantics Other sharingServiceNamed:)]
        pub unsafe fn sharingServiceNamed(
            service_name: &NSSharingServiceName,
        ) -> Option<Retained<NSSharingService>>;

        #[cfg(all(feature = "NSImage", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTitle:image:alternateImage:handler:)]
        pub unsafe fn initWithTitle_image_alternateImage_handler(
            this: Allocated<Self>,
            title: &NSString,
            image: &NSImage,
            alternate_image: Option<&NSImage>,
            block: &block2::Block<dyn Fn()>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(canPerformWithItems:)]
        pub unsafe fn canPerformWithItems(&self, items: Option<&NSArray>) -> bool;

        #[method(performWithItems:)]
        pub unsafe fn performWithItems(&self, items: &NSArray);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSharingService {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSharingContentScope(pub NSInteger);
impl NSSharingContentScope {
    #[doc(alias = "NSSharingContentScopeItem")]
    pub const Item: Self = Self(0);
    #[doc(alias = "NSSharingContentScopePartial")]
    pub const Partial: Self = Self(1);
    #[doc(alias = "NSSharingContentScopeFull")]
    pub const Full: Self = Self(2);
}

unsafe impl Encode for NSSharingContentScope {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSSharingContentScope {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSSharingServiceDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[optional]
        #[method(sharingService:willShareItems:)]
        unsafe fn sharingService_willShareItems(
            &self,
            sharing_service: &NSSharingService,
            items: &NSArray,
        );

        #[optional]
        #[method(sharingService:didFailToShareItems:error:)]
        unsafe fn sharingService_didFailToShareItems_error(
            &self,
            sharing_service: &NSSharingService,
            items: &NSArray,
            error: &NSError,
        );

        #[optional]
        #[method(sharingService:didShareItems:)]
        unsafe fn sharingService_didShareItems(
            &self,
            sharing_service: &NSSharingService,
            items: &NSArray,
        );

        #[optional]
        #[method(sharingService:sourceFrameOnScreenForShareItem:)]
        unsafe fn sharingService_sourceFrameOnScreenForShareItem(
            &self,
            sharing_service: &NSSharingService,
            item: &AnyObject,
        ) -> NSRect;

        #[cfg(feature = "NSImage")]
        #[optional]
        #[method_id(@__retain_semantics Other sharingService:transitionImageForShareItem:contentRect:)]
        unsafe fn sharingService_transitionImageForShareItem_contentRect(
            &self,
            sharing_service: &NSSharingService,
            item: &AnyObject,
            content_rect: NonNull<NSRect>,
        ) -> Option<Retained<NSImage>>;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[optional]
        #[method_id(@__retain_semantics Other sharingService:sourceWindowForShareItems:sharingContentScope:)]
        unsafe fn sharingService_sourceWindowForShareItems_sharingContentScope(
            &self,
            sharing_service: &NSSharingService,
            items: &NSArray,
            sharing_content_scope: NonNull<NSSharingContentScope>,
        ) -> Option<Retained<NSWindow>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method_id(@__retain_semantics Other anchoringViewForSharingService:showRelativeToRect:preferredEdge:)]
        unsafe fn anchoringViewForSharingService_showRelativeToRect_preferredEdge(
            &self,
            sharing_service: &NSSharingService,
            positioning_rect: NonNull<NSRect>,
            preferred_edge: NonNull<NSRectEdge>,
        ) -> Option<Retained<NSView>>;
    }

    unsafe impl ProtocolType for dyn NSSharingServiceDelegate {}
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCloudKitSharingServiceOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSCloudKitSharingServiceOptions: NSUInteger {
        const NSCloudKitSharingServiceStandard = 0;
        const NSCloudKitSharingServiceAllowPublic = 1<<0;
        const NSCloudKitSharingServiceAllowPrivate = 1<<1;
        const NSCloudKitSharingServiceAllowReadOnly = 1<<4;
        const NSCloudKitSharingServiceAllowReadWrite = 1<<5;
    }
}

unsafe impl Encode for NSCloudKitSharingServiceOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCloudKitSharingServiceOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSCloudSharingServiceDelegate: NSSharingServiceDelegate {
        #[optional]
        #[method(sharingService:didCompleteForItems:error:)]
        unsafe fn sharingService_didCompleteForItems_error(
            &self,
            sharing_service: &NSSharingService,
            items: &NSArray,
            error: Option<&NSError>,
        );

        #[optional]
        #[method(optionsForSharingService:shareProvider:)]
        unsafe fn optionsForSharingService_shareProvider(
            &self,
            cloud_kit_sharing_service: &NSSharingService,
            provider: &NSItemProvider,
        ) -> NSCloudKitSharingServiceOptions;
    }

    unsafe impl ProtocolType for dyn NSCloudSharingServiceDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSharingServicePicker;

    unsafe impl ClassType for NSSharingServicePicker {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSSharingServicePicker {}

extern_methods!(
    unsafe impl NSSharingServicePicker {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSSharingServicePickerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSharingServicePickerDelegate>>,
        );

        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(this: Allocated<Self>, items: &NSArray) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(showRelativeToRect:ofView:preferredEdge:)]
        pub unsafe fn showRelativeToRect_ofView_preferredEdge(
            &self,
            rect: NSRect,
            view: &NSView,
            preferred_edge: NSRectEdge,
        );

        #[method(close)]
        pub unsafe fn close(&self);

        #[cfg(feature = "NSMenuItem")]
        #[method_id(@__retain_semantics Other standardShareMenuItem)]
        pub unsafe fn standardShareMenuItem(&self, mtm: MainThreadMarker) -> Retained<NSMenuItem>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSharingServicePicker {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSSharingServicePickerDelegate: NSObjectProtocol {
        #[optional]
        #[method_id(@__retain_semantics Other sharingServicePicker:sharingServicesForItems:proposedSharingServices:)]
        unsafe fn sharingServicePicker_sharingServicesForItems_proposedSharingServices(
            &self,
            sharing_service_picker: &NSSharingServicePicker,
            items: &NSArray,
            proposed_services: &NSArray<NSSharingService>,
        ) -> Retained<NSArray<NSSharingService>>;

        #[optional]
        #[method_id(@__retain_semantics Other sharingServicePicker:delegateForSharingService:)]
        unsafe fn sharingServicePicker_delegateForSharingService(
            &self,
            sharing_service_picker: &NSSharingServicePicker,
            sharing_service: &NSSharingService,
            mtm: MainThreadMarker,
        ) -> Option<Retained<ProtocolObject<dyn NSSharingServiceDelegate>>>;

        #[optional]
        #[method(sharingServicePicker:didChooseSharingService:)]
        unsafe fn sharingServicePicker_didChooseSharingService(
            &self,
            sharing_service_picker: &NSSharingServicePicker,
            service: Option<&NSSharingService>,
        );
    }

    unsafe impl ProtocolType for dyn NSSharingServicePickerDelegate {}
);