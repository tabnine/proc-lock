mod macro_args;

use crate::macro_args::MacroArgs;
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ItemFn};

/// _⚠ Use with caution - this macro panics on error - See `Panics` section below._
///
/// Wraps the annotated function with a blocking lock that is released when
/// the function is returned.
///
/// # Args:
/// - `name`: The name of the lock. Can be any relative / absolute path.
/// - `absolute`: Indicates whether the provided `name` should be created at the [`temp_dir`](std::env::temp_dir())
/// or as an absolute path (at the root directory). Default is `false`.
/// # Example
/// ```rust
/// use proclock_macro::proclock;
///
/// #[proclock(name = "my_lock.lock", absolute = false, blocking = true)]
/// fn my_locked_function() {}
/// ```
/// # Panics
/// This macro will panic if the underlying locking function call fails.
#[proc_macro_attribute]
pub fn proclock(args: TokenStream, input: TokenStream) -> TokenStream {
    let options = syn::parse_macro_input!(args as AttributeArgs);
    let function = syn::parse_macro_input!(input as ItemFn);

    let MacroArgs { name, absolute } = match MacroArgs::from_list(&options) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let locking_code = quote! {
        use proclock_api::{lock, try_lock, LockPath};
        let lock_path = if #absolute {
            LockPath::FullPath(#name)
        } else {
            LockPath::Tmp(#name)
        };

        let _guard = lock(&lock_path).unwrap();
    };

    let ItemFn {
        attrs,
        vis,
        sig,
        block: body,
    } = &function;

    let result = quote! {
        #(#attrs)*
        #vis #sig {
            #locking_code
            #body
        }
    };

    let res: proc_macro::TokenStream = result.into();
    res
}
