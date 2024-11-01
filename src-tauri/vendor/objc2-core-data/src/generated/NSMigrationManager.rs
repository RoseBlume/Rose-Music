//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMigrationManager;

    unsafe impl ClassType for NSMigrationManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSMigrationManager {}

extern_methods!(
    unsafe impl NSMigrationManager {
        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Init initWithSourceModel:destinationModel:)]
        pub unsafe fn initWithSourceModel_destinationModel(
            this: Allocated<Self>,
            source_model: &NSManagedObjectModel,
            destination_model: &NSManagedObjectModel,
        ) -> Retained<Self>;

        #[cfg(feature = "NSMappingModel")]
        #[method(migrateStoreFromURL:type:options:withMappingModel:toDestinationURL:destinationType:destinationOptions:error:_)]
        pub unsafe fn migrateStoreFromURL_type_options_withMappingModel_toDestinationURL_destinationType_destinationOptions_error(
            &self,
            source_url: &NSURL,
            s_store_type: &NSString,
            s_options: Option<&NSDictionary>,
            mappings: Option<&NSMappingModel>,
            d_url: &NSURL,
            d_store_type: &NSString,
            d_options: Option<&NSDictionary>,
        ) -> Result<(), Retained<NSError>>;

        #[method(usesStoreSpecificMigrationManager)]
        pub unsafe fn usesStoreSpecificMigrationManager(&self) -> bool;

        #[method(setUsesStoreSpecificMigrationManager:)]
        pub unsafe fn setUsesStoreSpecificMigrationManager(
            &self,
            uses_store_specific_migration_manager: bool,
        );

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[cfg(feature = "NSMappingModel")]
        #[method_id(@__retain_semantics Other mappingModel)]
        pub unsafe fn mappingModel(&self) -> Retained<NSMappingModel>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other sourceModel)]
        pub unsafe fn sourceModel(&self) -> Retained<NSManagedObjectModel>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other destinationModel)]
        pub unsafe fn destinationModel(&self) -> Retained<NSManagedObjectModel>;

        #[cfg(feature = "NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other sourceContext)]
        pub unsafe fn sourceContext(&self) -> Retained<NSManagedObjectContext>;

        #[cfg(feature = "NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other destinationContext)]
        pub unsafe fn destinationContext(&self) -> Retained<NSManagedObjectContext>;

        #[cfg(all(feature = "NSEntityDescription", feature = "NSEntityMapping"))]
        #[method_id(@__retain_semantics Other sourceEntityForEntityMapping:)]
        pub unsafe fn sourceEntityForEntityMapping(
            &self,
            m_entity: &NSEntityMapping,
        ) -> Option<Retained<NSEntityDescription>>;

        #[cfg(all(feature = "NSEntityDescription", feature = "NSEntityMapping"))]
        #[method_id(@__retain_semantics Other destinationEntityForEntityMapping:)]
        pub unsafe fn destinationEntityForEntityMapping(
            &self,
            m_entity: &NSEntityMapping,
        ) -> Option<Retained<NSEntityDescription>>;

        #[cfg(all(feature = "NSEntityMapping", feature = "NSManagedObject"))]
        #[method(associateSourceInstance:withDestinationInstance:forEntityMapping:)]
        pub unsafe fn associateSourceInstance_withDestinationInstance_forEntityMapping(
            &self,
            source_instance: &NSManagedObject,
            destination_instance: &NSManagedObject,
            entity_mapping: &NSEntityMapping,
        );

        #[cfg(feature = "NSManagedObject")]
        #[method_id(@__retain_semantics Other destinationInstancesForEntityMappingNamed:sourceInstances:)]
        pub unsafe fn destinationInstancesForEntityMappingNamed_sourceInstances(
            &self,
            mapping_name: &NSString,
            source_instances: Option<&NSArray<NSManagedObject>>,
        ) -> Retained<NSArray<NSManagedObject>>;

        #[cfg(feature = "NSManagedObject")]
        #[method_id(@__retain_semantics Other sourceInstancesForEntityMappingNamed:destinationInstances:)]
        pub unsafe fn sourceInstancesForEntityMappingNamed_destinationInstances(
            &self,
            mapping_name: &NSString,
            destination_instances: Option<&NSArray<NSManagedObject>>,
        ) -> Retained<NSArray<NSManagedObject>>;

        #[cfg(feature = "NSEntityMapping")]
        #[method_id(@__retain_semantics Other currentEntityMapping)]
        pub unsafe fn currentEntityMapping(&self) -> Retained<NSEntityMapping>;

        #[method(migrationProgress)]
        pub unsafe fn migrationProgress(&self) -> c_float;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<NSDictionary>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);

        #[method(cancelMigrationWithError:)]
        pub unsafe fn cancelMigrationWithError(&self, error: &NSError);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMigrationManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);