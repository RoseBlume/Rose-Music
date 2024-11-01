//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIContentUnavailableTextProperties;

    unsafe impl ClassType for UIContentUnavailableTextProperties {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UIContentUnavailableTextProperties {}

unsafe impl NSCopying for UIContentUnavailableTextProperties {}

unsafe impl NSObjectProtocol for UIContentUnavailableTextProperties {}

unsafe impl NSSecureCoding for UIContentUnavailableTextProperties {}

extern_methods!(
    unsafe impl UIContentUnavailableTextProperties {
        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Retained<UIFont>;

        #[cfg(feature = "UIFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: &UIFont);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Retained<UIColor>;

        #[cfg(feature = "UIColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &UIColor);

        #[cfg(feature = "NSParagraphStyle")]
        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[method(numberOfLines)]
        pub unsafe fn numberOfLines(&self) -> NSInteger;

        #[method(setNumberOfLines:)]
        pub unsafe fn setNumberOfLines(&self, number_of_lines: NSInteger);

        #[method(adjustsFontSizeToFitWidth)]
        pub unsafe fn adjustsFontSizeToFitWidth(&self) -> bool;

        #[method(setAdjustsFontSizeToFitWidth:)]
        pub unsafe fn setAdjustsFontSizeToFitWidth(&self, adjusts_font_size_to_fit_width: bool);

        #[method(minimumScaleFactor)]
        pub unsafe fn minimumScaleFactor(&self) -> CGFloat;

        #[method(setMinimumScaleFactor:)]
        pub unsafe fn setMinimumScaleFactor(&self, minimum_scale_factor: CGFloat);

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        #[method(setAllowsDefaultTighteningForTruncation:)]
        pub unsafe fn setAllowsDefaultTighteningForTruncation(
            &self,
            allows_default_tightening_for_truncation: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIContentUnavailableTextProperties {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);