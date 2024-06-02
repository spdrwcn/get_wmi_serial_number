// src/lib.rs  
  
use serde::Deserialize;  
use wmi::{COMLibrary, WMIConnection};  
use std::error::Error;  
use std::boxed::Box;  
  
#[derive(Deserialize, Debug)]  
pub struct Win32BIOS {  
    pub serialnumber: String,  
    // 可以添加其他字段，如果需要的话  
}  
  
pub fn get_bios_serial_number() -> Result<String, Box<dyn Error>> {  
    let com_con = COMLibrary::new()?;  
    let wmi_con = WMIConnection::new(com_con.into())?;  
    // 使用 query 方法，并指定我们期望的返回类型为 Win32_BIOS  
    let results: Vec<Win32BIOS> = wmi_con.raw_query("SELECT SerialNumber FROM Win32_BIOS")?;  
    if !results.is_empty() {  
        // 取出第一个元素的序列号并返回  
        return Ok(results[0].serialnumber.clone());  
    } else {  
        return Err("No BIOS Serial Number found.".into());  
    }  
}  

