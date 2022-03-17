use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub(crate) struct MacroArgs {
    #[darling(default)]
    pub(crate) name: String,
    #[darling(default)]
    pub(crate) absolute: bool,
}
