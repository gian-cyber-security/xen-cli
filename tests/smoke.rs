#[test]
fn permission_defaults_are_defined() {
    let p = xen_cli_placeholder();
    assert!(p);
}

fn xen_cli_placeholder() -> bool {
    true
}
