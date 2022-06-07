use tap::vfile::{VFile, VFileBuilder};
use tap::error::RustructError;

use anyhow::Result;
use serde::{Serialize, Deserialize};

use windows_drives::BufferedPhysicalDrive;
use seek_bufread::BufReader;

#[derive(Debug,Serialize,Deserialize)]
pub struct DeviceVFileBuilder
{
  id : u8,
  size : u64,
}

impl DeviceVFileBuilder
{
  fn device_id(path : String) -> Result<u8>
  {
    if let Some(prefix) = path.strip_prefix("\\\\.\\PhysicalDrive")
    {
      if let Ok(id) = prefix.parse::<u8>()
      {
        return Ok(id)
      }
    }
    Err(RustructError::OpenFile("Invalid path".into()).into())
  }

  pub fn new(file_path : String) -> Result<(String, DeviceVFileBuilder)>
  {
     let id = Self::device_id(file_path)?;
     let drive = match BufferedPhysicalDrive::open(id)
     {
       Ok(drive) => drive,
       Err(err) =>  return Err(RustructError::OpenFile(err).into()),
     };

     Ok((format!("PhysicalDrive{}", id), DeviceVFileBuilder{ id , size : drive.size()}))
  }
}

#[typetag::serde]
impl VFileBuilder for DeviceVFileBuilder
{
  fn open(&self) -> Result<Box<dyn VFile>>
  {
    let drive = match BufferedPhysicalDrive::open(self.id)
    {
       //Windows device is unbuffered so we add a large buffer
       //but this doesn't improve much the read speed ...
       Ok(drive) => BufReader::with_capacity(20*1024*1024, drive),
       Err(err) => return Err(RustructError::OpenFile(err).into()),
    };
    Ok(Box::new(drive))
  }

  fn size(&self) -> u64
  { 
    self.size
  }
}
