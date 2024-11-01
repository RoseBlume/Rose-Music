//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait CLLocationManagerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "CLLocation", feature = "CLLocationManager"))]
        #[deprecated = "Implement -locationManager:didUpdateLocations: instead"]
        #[optional]
        #[method(locationManager:didUpdateToLocation:fromLocation:)]
        unsafe fn locationManager_didUpdateToLocation_fromLocation(
            &self,
            manager: &CLLocationManager,
            new_location: &CLLocation,
            old_location: &CLLocation,
        );

        #[cfg(all(feature = "CLLocation", feature = "CLLocationManager"))]
        #[optional]
        #[method(locationManager:didUpdateLocations:)]
        unsafe fn locationManager_didUpdateLocations(
            &self,
            manager: &CLLocationManager,
            locations: &NSArray<CLLocation>,
        );

        #[cfg(all(feature = "CLHeading", feature = "CLLocationManager"))]
        #[optional]
        #[method(locationManager:didUpdateHeading:)]
        unsafe fn locationManager_didUpdateHeading(
            &self,
            manager: &CLLocationManager,
            new_heading: &CLHeading,
        );

        #[cfg(feature = "CLLocationManager")]
        #[optional]
        #[method(locationManagerShouldDisplayHeadingCalibration:)]
        unsafe fn locationManagerShouldDisplayHeadingCalibration(
            &self,
            manager: &CLLocationManager,
        ) -> bool;

        #[cfg(all(feature = "CLLocationManager", feature = "CLRegion"))]
        #[optional]
        #[method(locationManager:didDetermineState:forRegion:)]
        unsafe fn locationManager_didDetermineState_forRegion(
            &self,
            manager: &CLLocationManager,
            state: CLRegionState,
            region: &CLRegion,
        );

        #[cfg(all(
            feature = "CLBeaconRegion",
            feature = "CLLocationManager",
            feature = "CLRegion"
        ))]
        #[deprecated]
        #[optional]
        #[method(locationManager:didRangeBeacons:inRegion:)]
        unsafe fn locationManager_didRangeBeacons_inRegion(
            &self,
            manager: &CLLocationManager,
            beacons: &NSArray<CLBeacon>,
            region: &CLBeaconRegion,
        );

        #[cfg(all(
            feature = "CLBeaconRegion",
            feature = "CLLocationManager",
            feature = "CLRegion"
        ))]
        #[deprecated]
        #[optional]
        #[method(locationManager:rangingBeaconsDidFailForRegion:withError:)]
        unsafe fn locationManager_rangingBeaconsDidFailForRegion_withError(
            &self,
            manager: &CLLocationManager,
            region: &CLBeaconRegion,
            error: &NSError,
        );

        #[cfg(all(
            feature = "CLBeaconIdentityCondition",
            feature = "CLBeaconIdentityConstraint",
            feature = "CLBeaconRegion",
            feature = "CLCondition",
            feature = "CLLocationManager"
        ))]
        #[optional]
        #[method(locationManager:didRangeBeacons:satisfyingConstraint:)]
        unsafe fn locationManager_didRangeBeacons_satisfyingConstraint(
            &self,
            manager: &CLLocationManager,
            beacons: &NSArray<CLBeacon>,
            beacon_constraint: &CLBeaconIdentityConstraint,
        );

        #[cfg(all(
            feature = "CLBeaconIdentityCondition",
            feature = "CLBeaconIdentityConstraint",
            feature = "CLCondition",
            feature = "CLLocationManager"
        ))]
        #[optional]
        #[method(locationManager:didFailRangingBeaconsForConstraint:error:)]
        unsafe fn locationManager_didFailRangingBeaconsForConstraint_error(
            &self,
            manager: &CLLocationManager,
            beacon_constraint: &CLBeaconIdentityConstraint,
            error: &NSError,
        );

        #[cfg(all(feature = "CLLocationManager", feature = "CLRegion"))]
        #[optional]
        #[method(locationManager:didEnterRegion:)]
        unsafe fn locationManager_didEnterRegion(
            &self,
            manager: &CLLocationManager,
            region: &CLRegion,
        );

        #[cfg(all(feature = "CLLocationManager", feature = "CLRegion"))]
        #[optional]
        #[method(locationManager:didExitRegion:)]
        unsafe fn locationManager_didExitRegion(
            &self,
            manager: &CLLocationManager,
            region: &CLRegion,
        );

        #[cfg(feature = "CLLocationManager")]
        #[optional]
        #[method(locationManager:didFailWithError:)]
        unsafe fn locationManager_didFailWithError(
            &self,
            manager: &CLLocationManager,
            error: &NSError,
        );

        #[cfg(all(feature = "CLLocationManager", feature = "CLRegion"))]
        #[optional]
        #[method(locationManager:monitoringDidFailForRegion:withError:)]
        unsafe fn locationManager_monitoringDidFailForRegion_withError(
            &self,
            manager: &CLLocationManager,
            region: Option<&CLRegion>,
            error: &NSError,
        );

        #[cfg(feature = "CLLocationManager")]
        #[deprecated]
        #[optional]
        #[method(locationManager:didChangeAuthorizationStatus:)]
        unsafe fn locationManager_didChangeAuthorizationStatus(
            &self,
            manager: &CLLocationManager,
            status: CLAuthorizationStatus,
        );

        #[cfg(feature = "CLLocationManager")]
        #[optional]
        #[method(locationManagerDidChangeAuthorization:)]
        unsafe fn locationManagerDidChangeAuthorization(&self, manager: &CLLocationManager);

        #[cfg(all(feature = "CLLocationManager", feature = "CLRegion"))]
        #[optional]
        #[method(locationManager:didStartMonitoringForRegion:)]
        unsafe fn locationManager_didStartMonitoringForRegion(
            &self,
            manager: &CLLocationManager,
            region: &CLRegion,
        );

        #[cfg(feature = "CLLocationManager")]
        #[optional]
        #[method(locationManagerDidPauseLocationUpdates:)]
        unsafe fn locationManagerDidPauseLocationUpdates(&self, manager: &CLLocationManager);

        #[cfg(feature = "CLLocationManager")]
        #[optional]
        #[method(locationManagerDidResumeLocationUpdates:)]
        unsafe fn locationManagerDidResumeLocationUpdates(&self, manager: &CLLocationManager);

        #[cfg(feature = "CLLocationManager")]
        #[optional]
        #[method(locationManager:didFinishDeferredUpdatesWithError:)]
        unsafe fn locationManager_didFinishDeferredUpdatesWithError(
            &self,
            manager: &CLLocationManager,
            error: Option<&NSError>,
        );

        #[cfg(all(feature = "CLLocationManager", feature = "CLVisit"))]
        #[optional]
        #[method(locationManager:didVisit:)]
        unsafe fn locationManager_didVisit(&self, manager: &CLLocationManager, visit: &CLVisit);
    }

    unsafe impl ProtocolType for dyn CLLocationManagerDelegate {}
);