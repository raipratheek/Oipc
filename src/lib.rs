#![allow(dead_code)]
mod posix_shmem_api;
mod memory_handler;



#[cfg(test)]
mod tests {
    use crate::posix_shmem_api::*;
    
    
    

    struct TestStruct
    {
        x : i32,
        y : String,
    }

    
    fn open_shm(name : &'static str, length: i64) -> ShmemPointer
    {
        let fd = create_shmem(name);
        let _result = ftruncate_shmem(fd, length);
        mmap_shmem(fd, length as usize,ShmemProt::ProtReadWrite, ShmemFlags::MapShared)
    }

    fn close_shm(shm:&ShmemPointer, name : &'static str, length: usize)
    {
        let _unmap = unmap_shmem(shm,length);
        let _result = unlink_shmem(name);
    }

    #[test]
    fn shm_open_close_test()
    {
        let fd = create_shmem("/test");
        assert!(fd > 0, "File descriptor is not greater than 0");
        let result = ftruncate_shmem(fd, 4092);
        assert!(result == 0, "Return value not equal to 0 for ftruncate");
        let virt_addr = mmap_shmem(fd, 4092,ShmemProt::ProtReadWrite, ShmemFlags::MapShared);
        let unmap = unmap_shmem(&virt_addr,4092);
        assert!(unmap == 0, "Return value not equal to 0 for unmap");
        let result = unlink_shmem("/test");
        assert!(result == 0, "Return value not equal to 0");
    }

    #[test]
    fn shm_read_write_test() {
       
        let name = "/test";
        let virt_addr = open_shm(&name, 64);
        assert!(virt_addr.write_to_offset(0, TestStruct{x:123, y:String::from("hello world!")}) != -1, "Write failed!");
        let read  = virt_addr.read_from_offset(0);
        let _teststr = String::from("hello world!");
        assert!(match read {
                Some(TestStruct{x:123,y:_teststr}) => true,
                _ => false,
        }, "The value read is not correct");
        println!(" shmem pointer looks like this {}", &virt_addr);

        close_shm(&virt_addr,&name,64) ;
    }


}
