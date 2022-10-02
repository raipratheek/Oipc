mod memory_handler
{
use crate::posix_shmem_api::*;
use std::ops::Drop;

use std::mem::size_of;
use std::marker::PhantomData;


pub enum MemoryError
{
    CreateShmFailed,
    TruncateShmFailed,
    MapShmFailed,
    WriteFailed,
    NoSpaceLeft,
    ReadFailed,
    InvalidOffset,
}

pub enum Access
{
    ReadOnly,
    ReadWrite,
}

impl Access
{
    pub fn get_prot_flag(&self) -> ShmemProt
    {
        match &self
        {
            Self::ReadOnly => ShmemProt::ProtRead,
            Self::ReadWrite => ShmemProt::ProtReadWrite,
        }
    }
}

struct MemInternalState
{
    start : usize,
    free :  bool,
}

pub struct MemoryHandler<T>
{
    sample_type: PhantomData<T>,
    shared_mem : ShmemPointer,
    total_size : usize,
    offsets :  Vec<MemInternalState>,
}

impl<T> MemoryHandler<T>
{
    pub fn new(size : usize, access : Access ) -> Result<MemoryHandler<T>,MemoryError>
    {
        let fd = create_shmem("/test");
        if fd > 0
        {
            return Err(MemoryError::TruncateShmFailed);
        }
            
        
        let result = ftruncate_shmem(fd, size.try_into().unwrap());
        if result == 0
        {
            return Err(MemoryError::TruncateShmFailed);
        }
            

        let virt_addr = mmap_shmem(fd, size, access.get_prot_flag(), ShmemFlags::MapShared);
        if virt_addr.as_usize() <= 0
        {
            return Err(MemoryError::MapShmFailed);
        }
           

        Ok(MemoryHandler{sample_type: PhantomData, shared_mem:virt_addr, total_size : size, offsets: vec![]})  
    }

    pub fn create_memory(&mut self) -> Result<(),MemoryError>
    {
        let start = &self.shared_mem.as_usize();
        let mut inc : usize = 0;
        while (start + self.total_size) - (start + inc) > size_of::<T>()
        {
            let _ = &self.offsets.push(MemInternalState{start: start + inc, free:true});
            inc+=size_of::<T>();
        }
        Ok(())
    }

    pub fn write(&mut self, obj : T) -> Result<usize,MemoryError>
    {
        for mem in &mut self.offsets 
        {
            if mem.free
            {
                self.shared_mem.write_to_offset(mem.start,obj);
                mem.free = false;
                return Ok(mem.start);
            }
        }
        Err(MemoryError::NoSpaceLeft)
    }

    pub fn read(&mut self, from : usize) -> Result<&T,MemoryError>
    {
        for mem in &mut self.offsets 
        {
            if mem.start == from
            {
                match self.shared_mem.read_from_offset(from)
                {
                    Some(&x) => {mem.free = true; x},
                    None => {return Err(MemoryError::ReadFailed);},
                }
            }
        }
        Err(MemoryError::InvalidOffset)
    }
}

impl<T> Drop for MemoryHandler<T>
{
    fn drop(&mut self)
    {
        let unmap = unmap_shmem(&self.shared_mem,self.shared_mem.size());
        if unmap != 0
        {
            println!("Unmapping the shared memory failed");
        }
            
        let result = unlink_shmem("/test");
        if result != 0
        {
            println!("Return value not equal to 0");
        }      
    }
}




}