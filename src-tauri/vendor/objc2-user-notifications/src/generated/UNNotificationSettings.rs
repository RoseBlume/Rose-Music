//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UNAuthorizationStatus(pub NSInteger);
impl UNAuthorizationStatus {
    #[doc(alias = "UNAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "UNAuthorizationStatusDenied")]
    pub const Denied: Self = Self(1);
    #[doc(alias = "UNAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(2);
    #[doc(alias = "UNAuthorizationStatusProvisional")]
    pub const Provisional: Self = Self(3);
    #[doc(alias = "UNAuthorizationStatusEphemeral")]
    pub const Ephemeral: Self = Self(4);
}

unsafe impl Encode for UNAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UNAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UNShowPreviewsSetting(pub NSInteger);
impl UNShowPreviewsSetting {
    #[doc(alias = "UNShowPreviewsSettingAlways")]
    pub const Always: Self = Self(0);
    #[doc(alias = "UNShowPreviewsSettingWhenAuthenticated")]
    pub const WhenAuthenticated: Self = Self(1);
    #[doc(alias = "UNShowPreviewsSettingNever")]
    pub const Never: Self = Self(2);
}

unsafe impl Encode for UNShowPreviewsSetting {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UNShowPreviewsSetting {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UNNotificationSetting(pub NSInteger);
impl UNNotificationSetting {
    #[doc(alias = "UNNotificationSettingNotSupported")]
    pub const NotSupported: Self = Self(0);
    #[doc(alias = "UNNotificationSettingDisabled")]
    pub const Disabled: Self = Self(1);
    #[doc(alias = "UNNotificationSettingEnabled")]
    pub const Enabled: Self = Self(2);
}

unsafe impl Encode for UNNotificationSetting {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UNNotificationSetting {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UNAlertStyle(pub NSInteger);
impl UNAlertStyle {
    #[doc(alias = "UNAlertStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UNAlertStyleBanner")]
    pub const Banner: Self = Self(1);
    #[doc(alias = "UNAlertStyleAlert")]
    pub const Alert: Self = Self(2);
}

unsafe impl Encode for UNAlertStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UNAlertStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationSettings;

    unsafe impl ClassType for UNNotificationSettings {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNNotificationSettings {}

unsafe impl NSCopying for UNNotificationSettings {}

unsafe impl NSObjectProtocol for UNNotificationSettings {}

unsafe impl NSSecureCoding for UNNotificationSettings {}

extern_methods!(
    unsafe impl UNNotificationSettings {
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus(&self) -> UNAuthorizationStatus;

        #[method(soundSetting)]
        pub unsafe fn soundSetting(&self) -> UNNotificationSetting;

        #[method(badgeSetting)]
        pub unsafe fn badgeSetting(&self) -> UNNotificationSetting;

        #[method(alertSetting)]
        pub unsafe fn alertSetting(&self) -> UNNotificationSetting;

        #[method(notificationCenterSetting)]
        pub unsafe fn notificationCenterSetting(&self) -> UNNotificationSetting;

        #[method(lockScreenSetting)]
        pub unsafe fn lockScreenSetting(&self) -> UNNotificationSetting;

        #[method(carPlaySetting)]
        pub unsafe fn carPlaySetting(&self) -> UNNotificationSetting;

        #[method(alertStyle)]
        pub unsafe fn alertStyle(&self) -> UNAlertStyle;

        #[method(showPreviewsSetting)]
        pub unsafe fn showPreviewsSetting(&self) -> UNShowPreviewsSetting;

        #[method(criticalAlertSetting)]
        pub unsafe fn criticalAlertSetting(&self) -> UNNotificationSetting;

        #[method(providesAppNotificationSettings)]
        pub unsafe fn providesAppNotificationSettings(&self) -> bool;

        #[method(announcementSetting)]
        pub unsafe fn announcementSetting(&self) -> UNNotificationSetting;

        #[method(timeSensitiveSetting)]
        pub unsafe fn timeSensitiveSetting(&self) -> UNNotificationSetting;

        #[method(scheduledDeliverySetting)]
        pub unsafe fn scheduledDeliverySetting(&self) -> UNNotificationSetting;

        #[method(directMessagesSetting)]
        pub unsafe fn directMessagesSetting(&self) -> UNNotificationSetting;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationSettings {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);