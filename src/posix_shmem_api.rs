
extern crate libc;

use libc::c_void;

use libc::size_t;
use libc::mode_t;

use std::fmt;


pub struct ShmemPointer
{
    fd : i32,
    ptr : *const c_void,
    size : usize,  
}

impl ShmemPointer
{
    pub fn read_from_offset<T>(&self, offset:usize) -> Option<&T>
    {
        match std::mem::size_of::<T>() <= (self.size - offset)
        {
            true => unsafe {Some::<&T>(&*((self.ptr as usize + offset) as *const c_void as *const T))},
            _ => None,
        }
        
    }

    pub fn write_to_offset<T>(&self, offset:usize, data: T) -> i32
    {
        match std::mem::size_of::<T>() <= (self.size - offset)
        {
            true => {
                    unsafe{*((self.ptr as usize + offset) as *mut c_void as *mut T) = data;};
                        0
                    },

            _ => -1,
        }
    }

    pub fn raw(&self) -> *const c_void
    {
        self.ptr
    }

    pub fn as_usize(&self) -> usize
    {
        self.ptr as usize
    }

    pub fn size(&self) -> usize
    {
        self.size as usize
    }
}

impl fmt::Display for ShmemPointer
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>
    {
        write!(f, "(fd = {}, size = {}, ptr = {})", self.fd, self.size, self.ptr as usize)
    }
}

pub enum ShmemProt
{
    ProtRead,
    ProtReadWrite,
}

impl ShmemProt
{
    fn value(&self) -> i32
    {
        match &self{
            ShmemProt::ProtRead => 1,
            ShmemProt::ProtReadWrite => 3, 
        }
    }

}

pub enum ShmemFlags
{
    MapShared,
}

impl ShmemFlags
{
    fn value(&self) -> i32
    {
        match &self
        {
            ShmemFlags::MapShared => 1,
        }
    }
}

extern "C" {
    fn shm_open(name: *const u8,oflag: i32, mode: mode_t) -> i32;
    fn shm_unlink(name: *const u8) -> i32;
    fn ftruncate(fd: i32, length: i64)->i32;
    fn mmap(addr : *mut c_void, length: size_t, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut c_void;
    fn munmap(addr : *mut c_void, length: size_t)->i32;
}

pub fn print_err_no()
{
    unsafe {
        let errno = *libc::__errno_location();
        println!("errno  {}", errno);
    }  
}

pub fn create_shmem(name: &'static str)-> i32
{
    unsafe
    {
        shm_open(name.as_ptr(),libc::O_RDWR | libc::O_CREAT, libc::S_IRUSR | libc::S_IWUSR)
    }
}

pub fn unlink_shmem(name: &'static str) -> i32
{
    unsafe {
        shm_unlink(name.as_ptr())
    }
}

pub fn ftruncate_shmem(fd: i32, length: i64) -> i32
{
    unsafe {
        ftruncate(fd, length) 
    }
}

pub fn mmap_shmem(fd: i32, length: usize, prot_flag : ShmemProt, map_flag : ShmemFlags) -> ShmemPointer
{
    unsafe {
        ShmemPointer{fd:fd,ptr:mmap(0 as *mut c_void,length, prot_flag.value(), map_flag.value(), fd, 0), size:length}
    } 
}

pub fn unmap_shmem(addr: &ShmemPointer, length: usize) -> i32
{
    unsafe {
        munmap(addr.raw() as *mut c_void, length)
    }
}



