use crate::get;
use crate::set;

pub trait CrossPlatformWireGuardClient {
    type GetError;
    type SetError;

    fn get(&mut self) -> Result<get::Device, Self::GetError>;
    fn set(&mut self, set_request: set::Device) -> Result<(), Self::SetError>;
}
