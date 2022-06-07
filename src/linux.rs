//! Device plugin let you load file from your filesystem in rustruct tree
//! It's simply a VFileBuilder wrapper around std:fs::File

use std::fs::{File};
use seek_bufread::BufReader;

use tap::vfile::{VFile, VFileBuilder};
use tap::error::RustructError;

use serde::{Serialize, Deserialize};
use nix::ioctl_read;
use std::os::unix::io::AsRawFd;
use std::fs::OpenOptions;

// Generate ioctl function
const BLKGETSIZE64_CODE: u8 = 0x12; // Defined in linux/fs.h
const BLKGETSIZE64_SEQ: u8 = 114;
ioctl_read!(ioctl_blkgetsize64, BLKGETSIZE64_CODE, BLKGETSIZE64_SEQ, u64);

/// Determine device size
fn get_device_size(path: &str) -> u64 
{
   let file = OpenOptions::new()
             .read(true)
             .open(path).unwrap();

   let fd = file.as_raw_fd();

   let mut cap = 0u64;
   let cap_ptr = &mut cap as *mut u64;

   unsafe 
   {
     ioctl_blkgetsize64(fd, cap_ptr).unwrap();
   }
  
   cap
}

#[derive(Debug,Serialize,Deserialize)]
pub struct DeviceVFileBuilder
{
  file_path : String,
  size : u64,
}

impl DeviceVFileBuilder
{
  pub fn new(file_path : String) -> anyhow::Result<(String, DeviceVFileBuilder)>
  {
     if File::open(file_path.clone()).is_ok() //we check if the file can be opened without error, to avoid error later
     {
       let device_name = file_path.split('/').last().unwrap();//XXX check error
       return Ok((device_name.to_string(), DeviceVFileBuilder{file_path : file_path.clone(), size : get_device_size(&file_path)}));
     }
     Err(RustructError::OpenFile(file_path).into()) //XXX return more precise error (permission error) 
  }
}

#[typetag::serde]
impl VFileBuilder for DeviceVFileBuilder
{
  fn open(&self) -> anyhow::Result<Box<dyn VFile>>
  {
    match File::open(&self.file_path)
    {
      Ok(file) => {
                    let file = BufReader::new(file);
                    Ok(Box::new(file))
                  },
      Err(_) => Err(RustructError::OpenFile(self.file_path.clone()).into()),
    }
  }

  fn size(&self) -> u64
  { 
    self.size
  }
}
