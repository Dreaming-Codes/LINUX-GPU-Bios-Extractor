use std::error::Error;
use std::process::Command;

fn get_multi_line_command_result(command: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let roms = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
    Ok(
        roms.stdout.split(|&b| b == b'\n')
            .map(|rom| {
                let rom = String::from_utf8(rom.to_vec()).unwrap();
                rom.trim_end_matches("/rom").to_string()
            })
            .filter(|gpu| gpu != "")
            .collect()
    )
}

pub mod getters{
    use std::error::Error;

    pub fn get_available_roms() -> Result<Vec<String>, Box<dyn Error>> {
        Ok(crate::get_multi_line_command_result("find /sys/devices -name rom")?)
    }

    pub fn get_available_gpus() -> Result<Vec<String>, Box<dyn Error>>{
        Ok(crate::get_multi_line_command_result("lspci | grep -i vga")?)
    }
}

pub fn extract_rom() {
}
