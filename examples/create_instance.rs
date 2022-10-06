use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::{Instance, InstanceCreateInfo, InstanceCreationError};
use vulkano_maybe_molten::NewMaybeMolten;

fn main() -> Result<(), InstanceCreationError> {
    let instance = Instance::new_maybe_molten(InstanceCreateInfo::default())?;

    let devices: Vec<_> = PhysicalDevice::enumerate(&instance)
        .map(|device| device.properties().device_name.clone())
        .collect();
    println!("Found GPUs: {:?}", devices);

    Ok(())
}
