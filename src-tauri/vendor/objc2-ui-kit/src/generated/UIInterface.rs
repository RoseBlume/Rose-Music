//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIBarStyle(pub NSInteger);
impl UIBarStyle {
    #[doc(alias = "UIBarStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "UIBarStyleBlack")]
    pub const Black: Self = Self(1);
    #[deprecated = "Use UIBarStyleBlack instead."]
    #[doc(alias = "UIBarStyleBlackOpaque")]
    pub const BlackOpaque: Self = Self(1);
    #[deprecated = "Use UIBarStyleBlack and set the translucent property to YES instead."]
    #[doc(alias = "UIBarStyleBlackTranslucent")]
    pub const BlackTranslucent: Self = Self(2);
}

unsafe impl Encode for UIBarStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIBarStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIUserInterfaceSizeClass(pub NSInteger);
impl UIUserInterfaceSizeClass {
    #[doc(alias = "UIUserInterfaceSizeClassUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "UIUserInterfaceSizeClassCompact")]
    pub const Compact: Self = Self(1);
    #[doc(alias = "UIUserInterfaceSizeClassRegular")]
    pub const Regular: Self = Self(2);
}

unsafe impl Encode for UIUserInterfaceSizeClass {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIUserInterfaceSizeClass {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIUserInterfaceStyle(pub NSInteger);
impl UIUserInterfaceStyle {
    #[doc(alias = "UIUserInterfaceStyleUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "UIUserInterfaceStyleLight")]
    pub const Light: Self = Self(1);
    #[doc(alias = "UIUserInterfaceStyleDark")]
    pub const Dark: Self = Self(2);
}

unsafe impl Encode for UIUserInterfaceStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIUserInterfaceStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIUserInterfaceLayoutDirection(pub NSInteger);
impl UIUserInterfaceLayoutDirection {
    #[doc(alias = "UIUserInterfaceLayoutDirectionLeftToRight")]
    pub const LeftToRight: Self = Self(0);
    #[doc(alias = "UIUserInterfaceLayoutDirectionRightToLeft")]
    pub const RightToLeft: Self = Self(1);
}

unsafe impl Encode for UIUserInterfaceLayoutDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIUserInterfaceLayoutDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITraitEnvironmentLayoutDirection(pub NSInteger);
impl UITraitEnvironmentLayoutDirection {
    #[doc(alias = "UITraitEnvironmentLayoutDirectionUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UITraitEnvironmentLayoutDirectionLeftToRight")]
    pub const LeftToRight: Self = Self(UIUserInterfaceLayoutDirection::LeftToRight.0);
    #[doc(alias = "UITraitEnvironmentLayoutDirectionRightToLeft")]
    pub const RightToLeft: Self = Self(UIUserInterfaceLayoutDirection::RightToLeft.0);
}

unsafe impl Encode for UITraitEnvironmentLayoutDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITraitEnvironmentLayoutDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDisplayGamut(pub NSInteger);
impl UIDisplayGamut {
    #[doc(alias = "UIDisplayGamutUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UIDisplayGamutSRGB")]
    pub const SRGB: Self = Self(0);
    #[doc(alias = "UIDisplayGamutP3")]
    pub const P3: Self = Self(1);
}

unsafe impl Encode for UIDisplayGamut {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDisplayGamut {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAccessibilityContrast(pub NSInteger);
impl UIAccessibilityContrast {
    #[doc(alias = "UIAccessibilityContrastUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UIAccessibilityContrastNormal")]
    pub const Normal: Self = Self(0);
    #[doc(alias = "UIAccessibilityContrastHigh")]
    pub const High: Self = Self(1);
}

unsafe impl Encode for UIAccessibilityContrast {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAccessibilityContrast {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UILegibilityWeight(pub NSInteger);
impl UILegibilityWeight {
    #[doc(alias = "UILegibilityWeightUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UILegibilityWeightRegular")]
    pub const Regular: Self = Self(0);
    #[doc(alias = "UILegibilityWeightBold")]
    pub const Bold: Self = Self(1);
}

unsafe impl Encode for UILegibilityWeight {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UILegibilityWeight {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIUserInterfaceLevel(pub NSInteger);
impl UIUserInterfaceLevel {
    #[doc(alias = "UIUserInterfaceLevelUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UIUserInterfaceLevelBase")]
    pub const Base: Self = Self(0);
    #[doc(alias = "UIUserInterfaceLevelElevated")]
    pub const Elevated: Self = Self(1);
}

unsafe impl Encode for UIUserInterfaceLevel {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIUserInterfaceLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIUserInterfaceActiveAppearance(pub NSInteger);
impl UIUserInterfaceActiveAppearance {
    #[doc(alias = "UIUserInterfaceActiveAppearanceUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UIUserInterfaceActiveAppearanceInactive")]
    pub const Inactive: Self = Self(0);
    #[doc(alias = "UIUserInterfaceActiveAppearanceActive")]
    pub const Active: Self = Self(1);
}

unsafe impl Encode for UIUserInterfaceActiveAppearance {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIUserInterfaceActiveAppearance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UINSToolbarItemPresentationSize(pub NSInteger);
impl UINSToolbarItemPresentationSize {
    #[doc(alias = "UINSToolbarItemPresentationSizeUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UINSToolbarItemPresentationSizeRegular")]
    pub const Regular: Self = Self(0);
    #[doc(alias = "UINSToolbarItemPresentationSizeSmall")]
    pub const Small: Self = Self(1);
    #[doc(alias = "UINSToolbarItemPresentationSizeLarge")]
    pub const Large: Self = Self(3);
}

unsafe impl Encode for UINSToolbarItemPresentationSize {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UINSToolbarItemPresentationSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImageDynamicRange(pub NSInteger);
impl UIImageDynamicRange {
    #[doc(alias = "UIImageDynamicRangeUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UIImageDynamicRangeStandard")]
    pub const Standard: Self = Self(0);
    #[doc(alias = "UIImageDynamicRangeConstrainedHigh")]
    pub const ConstrainedHigh: Self = Self(1);
    #[doc(alias = "UIImageDynamicRangeHigh")]
    pub const High: Self = Self(2);
}

unsafe impl Encode for UIImageDynamicRange {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImageDynamicRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// UIColorSystemColors
    #[cfg(feature = "UIColor")]
    unsafe impl UIColor {
        #[method_id(@__retain_semantics Other systemRedColor)]
        pub unsafe fn systemRedColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemGreenColor)]
        pub unsafe fn systemGreenColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemBlueColor)]
        pub unsafe fn systemBlueColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemOrangeColor)]
        pub unsafe fn systemOrangeColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemYellowColor)]
        pub unsafe fn systemYellowColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemPinkColor)]
        pub unsafe fn systemPinkColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemPurpleColor)]
        pub unsafe fn systemPurpleColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemTealColor)]
        pub unsafe fn systemTealColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemIndigoColor)]
        pub unsafe fn systemIndigoColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemBrownColor)]
        pub unsafe fn systemBrownColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemMintColor)]
        pub unsafe fn systemMintColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemCyanColor)]
        pub unsafe fn systemCyanColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemGrayColor)]
        pub unsafe fn systemGrayColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemGray2Color)]
        pub unsafe fn systemGray2Color() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemGray3Color)]
        pub unsafe fn systemGray3Color() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemGray4Color)]
        pub unsafe fn systemGray4Color() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemGray5Color)]
        pub unsafe fn systemGray5Color() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemGray6Color)]
        pub unsafe fn systemGray6Color() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other tintColor)]
        pub unsafe fn tintColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other labelColor)]
        pub unsafe fn labelColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other secondaryLabelColor)]
        pub unsafe fn secondaryLabelColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other tertiaryLabelColor)]
        pub unsafe fn tertiaryLabelColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other quaternaryLabelColor)]
        pub unsafe fn quaternaryLabelColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other linkColor)]
        pub unsafe fn linkColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other placeholderTextColor)]
        pub unsafe fn placeholderTextColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other separatorColor)]
        pub unsafe fn separatorColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other opaqueSeparatorColor)]
        pub unsafe fn opaqueSeparatorColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemBackgroundColor)]
        pub unsafe fn systemBackgroundColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other secondarySystemBackgroundColor)]
        pub unsafe fn secondarySystemBackgroundColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other tertiarySystemBackgroundColor)]
        pub unsafe fn tertiarySystemBackgroundColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemGroupedBackgroundColor)]
        pub unsafe fn systemGroupedBackgroundColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other secondarySystemGroupedBackgroundColor)]
        pub unsafe fn secondarySystemGroupedBackgroundColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other tertiarySystemGroupedBackgroundColor)]
        pub unsafe fn tertiarySystemGroupedBackgroundColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other systemFillColor)]
        pub unsafe fn systemFillColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other secondarySystemFillColor)]
        pub unsafe fn secondarySystemFillColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other tertiarySystemFillColor)]
        pub unsafe fn tertiarySystemFillColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other quaternarySystemFillColor)]
        pub unsafe fn quaternarySystemFillColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other lightTextColor)]
        pub unsafe fn lightTextColor() -> Retained<UIColor>;

        #[method_id(@__retain_semantics Other darkTextColor)]
        pub unsafe fn darkTextColor() -> Retained<UIColor>;

        #[deprecated]
        #[method_id(@__retain_semantics Other groupTableViewBackgroundColor)]
        pub unsafe fn groupTableViewBackgroundColor() -> Retained<UIColor>;

        #[deprecated]
        #[method_id(@__retain_semantics Other viewFlipsideBackgroundColor)]
        pub unsafe fn viewFlipsideBackgroundColor() -> Retained<UIColor>;

        #[deprecated]
        #[method_id(@__retain_semantics Other scrollViewTexturedBackgroundColor)]
        pub unsafe fn scrollViewTexturedBackgroundColor() -> Retained<UIColor>;

        #[deprecated]
        #[method_id(@__retain_semantics Other underPageBackgroundColor)]
        pub unsafe fn underPageBackgroundColor() -> Retained<UIColor>;
    }
);

extern_methods!(
    /// UIFontSystemFonts
    #[cfg(feature = "UIFont")]
    unsafe impl UIFont {
        #[method(labelFontSize)]
        pub unsafe fn labelFontSize() -> CGFloat;

        #[method(buttonFontSize)]
        pub unsafe fn buttonFontSize() -> CGFloat;

        #[method(smallSystemFontSize)]
        pub unsafe fn smallSystemFontSize() -> CGFloat;

        #[method(systemFontSize)]
        pub unsafe fn systemFontSize() -> CGFloat;

        #[method(defaultFontSize)]
        pub unsafe fn defaultFontSize() -> CGFloat;

        #[method(systemMinimumFontSize)]
        pub unsafe fn systemMinimumFontSize() -> CGFloat;
    }
);