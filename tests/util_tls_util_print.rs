use anyhow::Result;
use custom_utils::tls_util::print_cert;

#[test]
fn test() -> Result<()> {
    print_cert("./resource/certs/localhost.crt")?;
    Ok(())
}
