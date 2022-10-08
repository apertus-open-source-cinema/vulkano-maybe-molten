use std::sync::Arc;
use vulkano::instance::{Instance, InstanceCreateInfo, InstanceCreationError, InstanceExtensions};

pub trait NewMaybeMolten {
    fn new_maybe_molten(
        create_info: InstanceCreateInfo,
    ) -> Result<Arc<Instance>, InstanceCreationError>;
}
impl NewMaybeMolten for Instance {
    fn new_maybe_molten(
        create_info: InstanceCreateInfo,
    ) -> Result<Arc<Instance>, InstanceCreationError> {
        #[cfg(not(target_os = "macos"))]
        {
            Instance::new(create_info)
        }

        #[cfg(target_os = "macos")]
        {
            use std::os::raw::{c_char, c_void};
            use vulkano::instance::loader::{FunctionPointers, Loader};

            let extensions = InstanceExtensions {
                khr_surface: true,
                mvk_macos_surface: true,
                ..create_info.enabled_extensions
            };

            struct AshMoltenLoader {
                static_fn: ash_molten_version::vk::StaticFn,
            }
            unsafe impl Loader for AshMoltenLoader {
                fn get_instance_proc_addr(
                    &self,
                    instance: ash_vulkano_version::vk::Instance,
                    name: *const c_char,
                ) -> *const c_void {
                    let inner_result = unsafe {
                        self.static_fn
                            .get_instance_proc_addr(std::mem::transmute(instance), name)
                    };
                    if let Some(result) = inner_result {
                        result as *const c_void
                    } else {
                        0 as *const c_void
                    }
                }
            }

            let function_pointers: FunctionPointers<Box<dyn Loader>> =
                FunctionPointers::new(Box::new(AshMoltenLoader {
                    static_fn: ash_molten::load().static_fn().clone(),
                }));

            Instance::new(vulkano::instance::InstanceCreateInfo {
                enabled_extensions: extensions,
                function_pointers: Some(function_pointers),
                ..create_info
            })
        }
    }
}
