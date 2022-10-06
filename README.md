# `vulkano_maybe_molten`

Use vulkano-rs with ash-molten on macOS and without on other platforms.

This small crate allows you to use [vulkano-rs](https://github.com/vulkano-rs/vulkano) on macOS without needing to 
install the vulkan SDK by combining it with [ash-molten](https://github.com/EmbarkStudios/ash-molten). 
On non-apple platforms the vulkano-builtin ash is used.

## Usage

```rust
use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::{Instance, InstanceCreateInfo, InstanceCreationError};
use vulkano_maybe_molten::NewMaybeMolten;

fn main() -> Result<(), InstanceCreationError> {
    // the only thing we do differently here is to use new_maybe_molten() instead of new()
    let instance = Instance::new_maybe_molten(InstanceCreateInfo::default())?;

    let devices: Vec<_> = PhysicalDevice::enumerate(&instance)
        .map(|device| device.properties().device_name.clone())
        .collect();
    println!("Found GPUs: {:?}", devices);

    Ok(())
}
```
