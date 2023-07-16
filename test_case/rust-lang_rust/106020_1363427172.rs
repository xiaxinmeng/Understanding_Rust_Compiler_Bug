rust
> let instance_extensions = {
>     let surface_extensions =
>         ash_window::enumerate_required_extensions(window.raw_display_handle()).track()?;
> 
>     #[cfg(feature = "validation")]
>     let debug_utils_ext = vec![ash::extensions::ext::DebugUtils::name().as_ptr()];
> 
>     #[cfg(feature = "validation")]
>     { [debug_utils_ext, (*surface_extensions).to_owned()].concat() }
> 
>     #[cfg(not(feature = "validation"))]
>     { surface_extensions.to_owned() }
> };
> 