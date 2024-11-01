//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type NSTextContentType = NSString;

extern "C" {
    pub static NSTextContentTypeUsername: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypePassword: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeOneTimeCode: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeNewPassword: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeName: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeNamePrefix: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeGivenName: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeMiddleName: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeFamilyName: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeNameSuffix: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeNickname: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeJobTitle: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeOrganizationName: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeLocation: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeFullStreetAddress: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeStreetAddressLine1: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeStreetAddressLine2: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeAddressCity: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeAddressState: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeAddressCityAndState: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeSublocality: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCountryName: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypePostalCode: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeTelephoneNumber: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeEmailAddress: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeURL: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCreditCardNumber: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCreditCardName: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCreditCardGivenName: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCreditCardMiddleName: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCreditCardFamilyName: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCreditCardSecurityCode: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCreditCardExpiration: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCreditCardExpirationMonth: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCreditCardExpirationYear: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeCreditCardType: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeShipmentTrackingNumber: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeFlightNumber: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeDateTime: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeBirthdate: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeBirthdateDay: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeBirthdateMonth: &'static NSTextContentType;
}

extern "C" {
    pub static NSTextContentTypeBirthdateYear: &'static NSTextContentType;
}

extern_protocol!(
    pub unsafe trait NSTextContent {
        #[method_id(@__retain_semantics Other contentType)]
        unsafe fn contentType(&self) -> Option<Retained<NSTextContentType>>;

        #[method(setContentType:)]
        unsafe fn setContentType(&self, content_type: Option<&NSTextContentType>);
    }

    unsafe impl ProtocolType for dyn NSTextContent {}
);