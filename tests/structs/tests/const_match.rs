use test_structs::Windows::Win32::SystemServices::{NTSTATUS, E_NOTIMPL, DBG_TERMINATE_PROCESS};

#[test]
fn test() {
    match windows::HRESULT::default() {
        E_NOTIMPL => {}
        _ => {}
    }

    match NTSTATUS::default() {
        DBG_TERMINATE_PROCESS => {}
        _ => {}
    }
}
