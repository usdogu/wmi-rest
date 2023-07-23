use crate::Result;
pub async fn get_memory(machine_id: impl AsRef<str>, pwsh: &mut powershell_rs::Shell) -> Result {
    let machine_id = machine_id.as_ref();
    if machine_id.is_empty() {
        Err("No VM ID specified".into())
    } else if machine_id == "all" {
        let (sout, serr) = pwsh.execute(r#"Get-WmiObject -Namespace 'root\virtualization\v2' -Class Msvm_MemorySettingData -Filter "Caption like 'Memory'" | Select-Object -Property InstanceID, VirtualQuantity | ConvertTo-Json"#).await?;
        if !serr.is_empty() {
            Err(serr.into())
        } else {
            Ok(sout)
        }
    } else {
        let (sout, serr) = pwsh.execute(format!(r#"Get-WmiObject -Namespace 'root\virtualization\v2' -Class Msvm_MemorySettingData -Filter "Caption like 'Memory' AND InstanceID like '%{machine_id}%'" | Select-Object -Property InstanceID, VirtualQuantity | ConvertTo-Json"#)).await?;
        if !serr.is_empty() {
            Err(serr.into())
        } else {
            Ok(sout)
        }
    }
}