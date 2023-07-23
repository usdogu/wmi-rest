pub fn get_memory(machine_id: impl AsRef<str>, pwsh: &mut powershell_rs::Shell) -> Result<String, Box<dyn std::error::Error>> {
    let machine_id = machine_id.as_ref();
    if machine_id.is_empty() {
        Err("No VM ID specified".into())
    } else if machine_id == "all" {
        let (sout, serr) = pwsh.execute(r#"Get-WmiObject -Namespace 'root\virtualization\v2' -Class Msvm_MemorySettingData -Filter "Caption like 'Memory'" | Select-Object -Property InstanceID, VirtualQuantity | ConvertTo-Json"#)?;
        if !serr.is_empty() {
            Err(serr.into())
        } else {
            Ok(sout)
        }
    } else {
        // TODO: this is probably a bug, InstanceID and VirtualQuantity properties doesn't exists in the output of Get-VMMemory
        // let (sout, serr) = pwsh.execute(format!("Get-VM -Id {machine_id} | Get-VMMemory | Select-Object -Property InstanceID, VirtualQuantity | ConvertTo-Json"))?;
        let (sout, serr) = pwsh.execute(format!(r#"Get-WmiObject -Namespace 'root\virtualization\v2' -Class Msvm_MemorySettingData -Filter "Caption like 'Memory' AND InstanceID like '%{machine_id}%'" | Select-Object -Property InstanceID, VirtualQuantity | ConvertTo-Json"#))?;
        if !serr.is_empty() {
            Err(serr.into())
        } else {
            Ok(sout)
        }
    }
}