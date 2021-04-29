fn main() {
    windows::build!(
        // Test for https://github.com/microsoft/windows-rs/issues/738 as DebugPropertyInfo has a
        // BSTR field and needs to implement Clone.
        Windows::Win32::Debug::DebugPropertyInfo,

        // Test for https://github.com/microsoft/windows-rs/issues/739 to ensure constants of types
        // like NTSTATUS and HRESULT can be used in match expressions.
        Windows::Win32::SystemServices::{E_NOTIMPL, DBG_TERMINATE_PROCESS},
    );
}
