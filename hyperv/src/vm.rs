use crate::Result;
pub async fn get_vms(pwsh: &mut powershell_rs::Shell) -> Result {
    let (sout, serr) = pwsh.execute(r#"Get-WmiObject -namespace 'root\virtualization\v2' -class Msvm_ComputerSystem -Filter 'Caption="Virtual Machine"' | Select-Object -Property ElementName, InstallDate, Name, ProcessID | ConvertTo-Json"#).await?;
    if !serr.is_empty() {
        Err(serr.into())
    } else if sout.is_empty() {
        Err("No VMs found".into())
    } else {
        Ok(sout)
    }
}