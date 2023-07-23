use crate::Result;
use anyhow::bail;

pub async fn get_processor(machine_id: impl AsRef<str>, pwsh: &mut powershell_rs::Shell) -> Result {
    let machine_id = machine_id.as_ref();
    if machine_id.is_empty() {
        bail!("No VM ID specified");
    }
    if machine_id == "all" {
        let (sout, serr) = pwsh.execute(r#"Get-WmiObject -namespace 'root\virtualization\v2' -class Msvm_SummaryInformation | Select-Object -Property ElementName, InstanceID, NumberOfProcessors | ConvertTo-Json"#).await?;
        if !serr.is_empty() {
            bail!(serr);
        }
        return Ok(sout);
    }
    let (sout, serr) = pwsh
        .execute(format!(
            "Get-VM -Id {machine_id} | Get-VMProcessor | ConvertTo-Json"
        ))
        .await?;
    if !serr.is_empty() {
        bail!(serr);
    }
    Ok(sout)
}
