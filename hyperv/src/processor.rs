use crate::Result;
pub async fn get_processor(machine_id: impl AsRef<str>, pwsh: &mut powershell_rs::Shell) -> Result {
    let machine_id = machine_id.as_ref();
    if machine_id.is_empty() {
        return Err("No VM ID specified".into());
    }
    if machine_id == "all" {
        let (sout, serr) = pwsh.execute(r#"Get-WmiObject -namespace 'root\virtualization\v2' -class Msvm_SummaryInformation | Select-Object -Property ElementName, InstanceID, NumberOfProcessors | ConvertTo-Json"#).await?;
        if !serr.is_empty() {
            return Err(serr.into());
        }
        return Ok(sout);
    }
    let (sout, serr) = pwsh.execute(format!("Get-VM -Id {machine_id} | Get-VMProcessor | ConvertTo-Json")).await?;
    if !serr.is_empty() {
        return Err(serr.into());
    }
    Ok(sout)
}