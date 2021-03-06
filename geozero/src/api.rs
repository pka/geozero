//! API traits for reading datasets and features with geomeries.
//!
//! Features are usually consumed by datasource iterators.
//! The current feature can be processed with `FeatureAccess` processing API methods.
//! Some datasources process features during consumation (e.g. reading from file).

use crate::error::Result;
use crate::feature_processor::FeatureProcessor;
use crate::property_processor::{
    PropertyProcessor, PropertyReadType, PropertyReader, PropertyReaderIdx,
};
use crate::{CoordDimensions, GeomProcessor};
use std::collections::HashMap;

/// Geometry processing trait.
pub trait GeozeroGeometry {
    /// Process geometry.
    fn process_geom<P: GeomProcessor>(&self, processor: &mut P) -> Result<()>
    where
        Self: Sized;
    #[deprecated(
        since = "0.7.1",
        note = "please remove implementation, not required anymore"
    )]
    #[doc(hidden)]
    fn empty() -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
    /// Dimensions of geometry
    fn dims(&self) -> CoordDimensions {
        CoordDimensions::xy()
    }
    /// SRID of geometry
    fn srid(&self) -> Option<i32> {
        None
    }
}

/// Datasource feature consumer trait.
pub trait GeozeroDatasource {
    /// Consume and process all selected features.
    fn process<P: FeatureProcessor>(&mut self, processor: &mut P) -> Result<()>;
    /// Consume and process geometries of all selected features.
    fn process_geom<P: GeomProcessor>(&mut self, _processor: &mut P) -> Result<()> {
        unimplemented!() // will be mandatory in 0.8
    }
}

/// Feature processing API
pub trait FeatureAccess: FeatureProperties + GeozeroGeometry {
    /// Process feature geometries and properties.
    fn process<P: FeatureProcessor>(&self, processor: &mut P, idx: u64) -> Result<()>
    where
        Self: Sized,
    {
        processor.feature_begin(idx)?;
        processor.properties_begin()?;
        let _ = self.process_properties(processor)?;
        processor.properties_end()?;
        processor.geometry_begin()?;
        self.process_geom(processor)?;
        processor.geometry_end()?;
        processor.feature_end(idx)
    }
}

/// Feature properties processing API
pub trait FeatureProperties {
    /// Process feature properties.
    fn process_properties<P: PropertyProcessor>(&self, processor: &mut P) -> Result<bool>;
    /// Get property value by name
    fn property<T: PropertyReadType>(&self, name: &str) -> Option<T> {
        let mut reader = PropertyReader { name, value: None };
        if self.process_properties(&mut reader).is_ok() {
            reader.value
        } else {
            None
        }
    }
    /// Get property value by number
    fn property_n<T: PropertyReadType>(&self, n: usize) -> Option<T> {
        let mut reader = PropertyReaderIdx {
            idx: n,
            value: None,
        };
        if self.process_properties(&mut reader).is_ok() {
            reader.value
        } else {
            None
        }
    }
    /// Return all properties in a HashMap
    /// Use `process_properties` for zero-copy access
    fn properties(&self) -> Result<HashMap<String, String>> {
        let mut properties = HashMap::new();
        let _ = self.process_properties(&mut properties)?;
        Ok(properties)
    }
}
