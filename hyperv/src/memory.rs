pub fn get_memory(machine_id: impl AsRef<str>, pwsh: &mut powershell_rs::Shell) -> Result<String, Box<dyn std::error::Error>> {
    let machine_id = machine_id.as_ref();
    if machine_id.is_empty() {
        Ok("No VM ID specified".into())
    } else if machine_id == "all" {
        // FIXME: bellek -> memory
        let (sout, serr) = pwsh.execute(r#"Get-WmiObject -Namespace 'root\virtualization\v2' -Class Msvm_MemorySettingData -Filter "Caption like 'Bellek'" | Select-Object -Property InstanceID, VirtualQuantity | ConvertTo-Json"#)?;
        if !serr.is_empty() {
            Err(serr.into())
        } else {
            Ok(sout)
        }
    } else {
        let (sout, serr) = pwsh.execute(format!("Get-VM -Id {} | Get-VMMemory | Select-Object -Property InstanceID, VirtualQuantity | ConvertTo-Json", machine_id))?;
        if !serr.is_empty() {
            Err(serr.into())
        } else {
            Ok(sout)
        }
    }
}