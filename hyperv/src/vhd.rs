use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct VHDPathElement {
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "VMId")]
    vm_id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct VHDInfo {
    size: u64,
    #[serde(default)]
    id: String
}

pub fn get_vhd(machine_id: impl AsRef<str>, pwsh: &mut powershell_rs::Shell) -> Result<String, Box<dyn std::error::Error>> {
    let machine_id = machine_id.as_ref();
    if machine_id.is_empty() {
        return Err("No VM ID specified".into());
    }
    if machine_id == "all" {
        return get_all_vhd_info(pwsh);
    }
    get_vhd_info(machine_id, pwsh)
}

fn get_all_vhd_info(pwsh: &mut powershell_rs::Shell) -> Result<String, Box<dyn std::error::Error>> {
    let mut size_list = vec![];
    let (sout, serr) = pwsh.execute(r#"Get-VM | Get-VMHardDiskDrive | Select-Object -Property Path, VMId | ConvertTo-Json"#)?;
    if !serr.is_empty() {
        return Err(serr.into());
    }
    let vhd_path = serde_json::from_str::<Vec<VHDPathElement>>(&sout)?;
    for path in vhd_path {
        let (sout, serr) = pwsh.execute(format!(r#"Get-VHD -Path "{}" | Select-Object -Property Size | ConvertTo-Json"#, path.path))?;
        if !serr.is_empty() {
            return Err(serr.into());
        }
        let mut vhd_info = serde_json::from_str::<VHDInfo>(&sout)?;
        vhd_info.id = path.vm_id;
        size_list.push(vhd_info);
    }
    Ok(serde_json::to_string(&size_list)?)
}

fn get_vhd_info(machine_id: &str, pwsh: &mut powershell_rs::Shell) -> Result<String, Box<dyn std::error::Error>> {
    let (sout, serr) = pwsh.execute(format!(r#"Get-VHD -Id "{machine_id}" | ConvertTo-Json"#))?;
    if !serr.is_empty() {
        return Err(serr.into());
    }
    if sout.is_empty() {
        return Err("No Disk found".into());
    }
    Ok(sout)
}