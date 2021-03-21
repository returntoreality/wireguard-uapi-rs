use crate::get;
use crate::linux::err::GetDeviceError;
use crate::linux::err::SetDeviceError;
use crate::linux::DeviceInterface;
use crate::linux::WgSocket;
use crate::set;
use crate::xplatform::client::CrossPlatformWireGuardClient;

pub struct CrossPlatformLinuxClient<'a> {
    socket: WgSocket,
    interface: DeviceInterface<'a>,
}

impl CrossPlatformWireGuardClient for CrossPlatformLinuxClient<'_> {
    type GetError = GetDeviceError;
    type SetError = SetDeviceError;

    fn get(&mut self) -> Result<get::Device, Self::GetError> {
        self.socket.get_device(self.interface.clone())
    }

    fn set(&mut self, device: set::Device) -> Result<(), Self::SetError> {
        self.socket.set_device(self.interface.clone(), device)
    }
}
