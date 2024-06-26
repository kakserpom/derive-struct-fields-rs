pub struct StructFieldInfo {
    pub name: &'static str,
    pub field_type: &'static str,
}
pub trait StructFields {
    fn struct_fields() -> Vec<StructFieldInfo>;
    fn struct_field_names() -> Vec<String>;
}
