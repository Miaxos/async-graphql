use crate::{registry, Object};

pub struct __EnumValue<'a> {
    pub registry: &'a registry::Registry,
    pub value: &'a registry::MetaEnumValue,
}

/// One possible value for a given Enum. Enum values are unique values, not a placeholder for a string or numeric value. However an Enum value is returned in a JSON response as a string.
#[Object(internal, name = "__EnumValue")]
impl<'a> __EnumValue<'a> {
    async fn name(&self) -> String {
        self.value.name.to_string()
    }

    async fn description(&self) -> Option<String> {
        self.value.description.clone()
    }

    async fn is_deprecated(&self) -> bool {
        self.value.deprecation.is_some()
    }

    async fn deprecation_reason(&self) -> Option<String> {
        self.value.deprecation.clone()
    }
}
