//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    #[deprecated]
    pub struct CKFetchRecordChangesOperation;

    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl ClassType for CKFetchRecordChangesOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
unsafe impl NSObjectProtocol for CKFetchRecordChangesOperation {}

extern_methods!(
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchRecordChangesOperation {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "CKRecordZoneID", feature = "CKServerChangeToken"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithRecordZoneID:previousServerChangeToken:)]
        pub unsafe fn initWithRecordZoneID_previousServerChangeToken(
            this: Allocated<Self>,
            record_zone_id: &CKRecordZoneID,
            previous_server_change_token: Option<&CKServerChangeToken>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[deprecated]
        #[method_id(@__retain_semantics Other recordZoneID)]
        pub unsafe fn recordZoneID(&self) -> Option<Retained<CKRecordZoneID>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[deprecated]
        #[method(setRecordZoneID:)]
        pub unsafe fn setRecordZoneID(&self, record_zone_id: Option<&CKRecordZoneID>);

        #[cfg(feature = "CKServerChangeToken")]
        #[deprecated]
        #[method_id(@__retain_semantics Other previousServerChangeToken)]
        pub unsafe fn previousServerChangeToken(&self) -> Option<Retained<CKServerChangeToken>>;

        #[cfg(feature = "CKServerChangeToken")]
        #[deprecated]
        #[method(setPreviousServerChangeToken:)]
        pub unsafe fn setPreviousServerChangeToken(
            &self,
            previous_server_change_token: Option<&CKServerChangeToken>,
        );

        #[deprecated]
        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[deprecated]
        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[cfg(feature = "CKRecord")]
        #[deprecated]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Retained<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "CKRecord")]
        #[deprecated]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);

        #[cfg(all(feature = "CKRecord", feature = "block2"))]
        #[deprecated]
        #[method(recordChangedBlock)]
        pub unsafe fn recordChangedBlock(&self) -> *mut block2::Block<dyn Fn(NonNull<CKRecord>)>;

        #[cfg(all(feature = "CKRecord", feature = "block2"))]
        #[deprecated]
        #[method(setRecordChangedBlock:)]
        pub unsafe fn setRecordChangedBlock(
            &self,
            record_changed_block: Option<&block2::Block<dyn Fn(NonNull<CKRecord>)>>,
        );

        #[cfg(all(feature = "CKRecordID", feature = "block2"))]
        #[deprecated]
        #[method(recordWithIDWasDeletedBlock)]
        pub unsafe fn recordWithIDWasDeletedBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordID>)>;

        #[cfg(all(feature = "CKRecordID", feature = "block2"))]
        #[deprecated]
        #[method(setRecordWithIDWasDeletedBlock:)]
        pub unsafe fn setRecordWithIDWasDeletedBlock(
            &self,
            record_with_id_was_deleted_block: Option<&block2::Block<dyn Fn(NonNull<CKRecordID>)>>,
        );

        #[deprecated]
        #[method(moreComing)]
        pub unsafe fn moreComing(&self) -> bool;

        #[cfg(all(feature = "CKServerChangeToken", feature = "block2"))]
        #[deprecated]
        #[method(fetchRecordChangesCompletionBlock)]
        pub unsafe fn fetchRecordChangesCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut CKServerChangeToken, *mut NSData, *mut NSError)>;

        #[cfg(all(feature = "CKServerChangeToken", feature = "block2"))]
        #[deprecated]
        #[method(setFetchRecordChangesCompletionBlock:)]
        pub unsafe fn setFetchRecordChangesCompletionBlock(
            &self,
            fetch_record_changes_completion_block: Option<
                &block2::Block<dyn Fn(*mut CKServerChangeToken, *mut NSData, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchRecordChangesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);