use crate::Result;
use anyhow::bail;
pub async fn get_vms(pwsh: &mut powershell_rs::Shell) -> Result {
    let (sout, serr) = pwsh.execute(r#"Get-WmiObject -namespace 'root\virtualization\v2' -class Msvm_ComputerSystem -Filter 'Caption="Virtual Machine"' | Select-Object -Property ElementName, InstallDate, Name, ProcessID | ConvertTo-Json"#).await?;
    if !serr.is_empty() {
        bail!(serr)
    } else if sout.is_empty() {
        bail!("No VMs found")
    } else {
        Ok(sout)
    }
}
