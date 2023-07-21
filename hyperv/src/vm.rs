pub fn get_vms(pwsh: &mut powershell_rs::Shell) -> Result<String, Box<dyn std::error::Error>> {
    // FIXME: sanal makine -> virtual machine
    let (sout, serr) = pwsh.execute(r#"Get-WmiObject -namespace 'root\virtualization\v2' -class Msvm_ComputerSystem -Filter 'Caption="Sanal Makine"' | Select-Object -Property ElementName, InstallDate, Name, ProcessID | ConvertTo-Json"#)?;
    if !serr.is_empty() {
        Err(serr.into())
    } else if sout.is_empty() {
        Err("No VMs found".into())
    } else {
        Ok(sout)
    }
}