//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPushBehaviorMode(pub NSInteger);
impl UIPushBehaviorMode {
    #[doc(alias = "UIPushBehaviorModeContinuous")]
    pub const Continuous: Self = Self(0);
    #[doc(alias = "UIPushBehaviorModeInstantaneous")]
    pub const Instantaneous: Self = Self(1);
}

unsafe impl Encode for UIPushBehaviorMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPushBehaviorMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDynamicBehavior")]
    pub struct UIPushBehavior;

    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl ClassType for UIPushBehavior {
        #[inherits(NSObject)]
        type Super = UIDynamicBehavior;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIDynamicBehavior")]
unsafe impl NSObjectProtocol for UIPushBehavior {}

extern_methods!(
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIPushBehavior {
        #[method_id(@__retain_semantics Init initWithItems:mode:)]
        pub unsafe fn initWithItems_mode(
            this: Allocated<Self>,
            items: &NSArray<ProtocolObject<dyn UIDynamicItem>>,
            mode: UIPushBehaviorMode,
        ) -> Retained<Self>;

        #[method(addItem:)]
        pub unsafe fn addItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Retained<NSArray<ProtocolObject<dyn UIDynamicItem>>>;

        #[cfg(feature = "UIGeometry")]
        #[method(targetOffsetFromCenterForItem:)]
        pub unsafe fn targetOffsetFromCenterForItem(
            &self,
            item: &ProtocolObject<dyn UIDynamicItem>,
        ) -> UIOffset;

        #[cfg(feature = "UIGeometry")]
        #[method(setTargetOffsetFromCenter:forItem:)]
        pub unsafe fn setTargetOffsetFromCenter_forItem(
            &self,
            o: UIOffset,
            item: &ProtocolObject<dyn UIDynamicItem>,
        );

        #[method(mode)]
        pub unsafe fn mode(&self) -> UIPushBehaviorMode;

        #[method(active)]
        pub unsafe fn active(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[method(angle)]
        pub unsafe fn angle(&self) -> CGFloat;

        #[method(setAngle:)]
        pub unsafe fn setAngle(&self, angle: CGFloat);

        #[method(magnitude)]
        pub unsafe fn magnitude(&self) -> CGFloat;

        #[method(setMagnitude:)]
        pub unsafe fn setMagnitude(&self, magnitude: CGFloat);

        #[method(pushDirection)]
        pub unsafe fn pushDirection(&self) -> CGVector;

        #[method(setPushDirection:)]
        pub unsafe fn setPushDirection(&self, push_direction: CGVector);

        #[method(setAngle:magnitude:)]
        pub unsafe fn setAngle_magnitude(&self, angle: CGFloat, magnitude: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIPushBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);