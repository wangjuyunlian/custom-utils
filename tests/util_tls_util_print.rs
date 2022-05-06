use anyhow::Result;
use custom_utils::print_cert;
#[test]
fn test() -> Result<()> {
    // print_cert("./resource/certs/root.crt")?;
    print_cert("./resource/certs/localhost.crt")?;
    Ok(())
}
