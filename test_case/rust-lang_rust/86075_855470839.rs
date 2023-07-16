rust
#![feature(with_options)]

use std::path::Path;

mod c {
    #[repr(C)]
    pub enum FILE_INFO_BY_HANDLE_CLASS {
        FileBasicInfo = 0,
        FileStandardInfo = 1,
        FileNameInfo = 2,
        FileRenameInfo = 3,
        FileDispositionInfo = 4,
        FileAllocationInfo = 5,
        FileEndOfFileInfo = 6,
        FileStreamInfo = 7,
        FileCompressionInfo = 8,
        FileAttributeTagInfo = 9,
        FileIdBothDirectoryInfo = 10,        // 0xA
        FileIdBothDirectoryRestartInfo = 11, // 0xB
        FileIoPriorityHintInfo = 12,         // 0xC
        FileRemoteProtocolInfo = 13,         // 0xD
        FileFullDirectoryInfo = 14,          // 0xE
        FileFullDirectoryRestartInfo = 15,   // 0xF
        FileStorageInfo = 16,                // 0x10
        FileAlignmentInfo = 17,              // 0x11
        FileIdInfo = 18,                     // 0x12
        FileIdExtdDirectoryInfo = 19,        // 0x13
        FileIdExtdDirectoryRestartInfo = 20, // 0x14
        MaximumFileInfoByHandlesClass,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct FILE_ID_INFO {
        pub VolumeSerialNumber: u64,
        pub FileId: u128
    }

    extern "system" {
        pub fn GetFileInformationByHandleEx(
            hFile: *mut std::ffi::c_void,
            fileInfoClass: FILE_INFO_BY_HANDLE_CLASS,
            lpFileInformation: *mut std::ffi::c_void,
            dwBufferSize: u32,
        ) -> i32;
    }
}

fn main() {
    unsafe {
        use std::os::windows::prelude::*;

        let handle = std::fs::File::with_options().access_mode(0).open("a/real.file").unwrap().into_raw_handle();

        let mut info: c::FILE_ID_INFO = std::mem::zeroed();
        let result = c::GetFileInformationByHandleEx(
            handle,
            c::FILE_INFO_BY_HANDLE_CLASS::FileIdInfo,
            &mut info as *mut _ as *mut std::ffi::c_void,
            std::mem::size_of_val(&info) as u32
        );

        if result == 0 {
            panic!("{}", std::io::Error::last_os_error())
        } else {
            // prints "FILE_ID_INFO { VolumeSerialNumber: xxxxxxxxxxxxxxxxx, FileId: xxxxxxxxxxxxxxxxx }" on Windows 10
            println!("{:?}", info);
        }
    }
}
