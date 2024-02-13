/*
 * ByRefOrValue
 *
 * This tests for a oneOf interface representation 
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use super::Bar;
use super::BarRef;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BarRefOrValue {
    Bar(Box<Bar>),
    BarRef(Box<BarRef>),
}

impl Default for BarRefOrValue {
    fn default() -> Self {
        Self::Bar(Box::default())
    }
}


