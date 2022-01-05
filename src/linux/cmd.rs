use neli::neli_enum;

// https://github.com/WireGuard/WireGuard/blob/62b335b56cc99312ccedfa571500fbef3756a623/src/uapi/wireguard.h#L137
#[neli_enum(serialized_type = "u8")]
pub(crate) enum WgCmd {
    GetDevice = 1,
    SetDevice = 2,
}

impl neli::consts::genl::Cmd for WgCmd {}