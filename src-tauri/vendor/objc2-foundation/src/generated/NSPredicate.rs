//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPredicate;

    unsafe impl ClassType for NSPredicate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSPredicate {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSPredicate {}

unsafe impl NSObjectProtocol for NSPredicate {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSPredicate {}

extern_methods!(
    unsafe impl NSPredicate {
        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other predicateWithFormat:argumentArray:)]
        pub unsafe fn predicateWithFormat_argumentArray(
            predicate_format: &NSString,
            arguments: Option<&NSArray>,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other predicateFromMetadataQueryString:)]
        pub unsafe fn predicateFromMetadataQueryString(
            query_string: &NSString,
        ) -> Option<Retained<NSPredicate>>;

        #[method_id(@__retain_semantics Other predicateWithValue:)]
        pub unsafe fn predicateWithValue(value: bool) -> Retained<NSPredicate>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString", feature = "block2"))]
        #[method_id(@__retain_semantics Other predicateWithBlock:)]
        pub unsafe fn predicateWithBlock(
            block: &block2::Block<
                dyn Fn(*mut AnyObject, *mut NSDictionary<NSString, AnyObject>) -> Bool,
            >,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other predicateFormat)]
        pub unsafe fn predicateFormat(&self) -> Retained<NSString>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other predicateWithSubstitutionVariables:)]
        pub unsafe fn predicateWithSubstitutionVariables(
            &self,
            variables: &NSDictionary<NSString, AnyObject>,
        ) -> Retained<Self>;

        #[method(evaluateWithObject:)]
        pub unsafe fn evaluateWithObject(&self, object: Option<&AnyObject>) -> bool;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(evaluateWithObject:substitutionVariables:)]
        pub unsafe fn evaluateWithObject_substitutionVariables(
            &self,
            object: Option<&AnyObject>,
            bindings: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> bool;

        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPredicate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "NSArray")]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method_id(@__retain_semantics Other filteredArrayUsingPredicate:)]
        pub unsafe fn filteredArrayUsingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Retained<NSArray<ObjectType>>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "NSArray")]
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&mut self, predicate: &NSPredicate);
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "NSSet")]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[method_id(@__retain_semantics Other filteredSetUsingPredicate:)]
        pub unsafe fn filteredSetUsingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Retained<NSSet<ObjectType>>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "NSSet")]
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&mut self, predicate: &NSPredicate);
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics Other filteredOrderedSetUsingPredicate:)]
        pub unsafe fn filteredOrderedSetUsingPredicate(
            &self,
            p: &NSPredicate,
        ) -> Retained<NSOrderedSet<ObjectType>>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&mut self, p: &NSPredicate);
    }
);