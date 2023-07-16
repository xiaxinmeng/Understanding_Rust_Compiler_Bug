rust
struct Statement {
    stmt: NotNull<ffi::MYSQL_STMT>,
    input_binds: Option<Binds>
}

pub struct Binds {
    data: Vec<BindData>,
}

pub struct BindData {
    tpe: ffi::enum_field_types,
    bytes: Vec<u8>,
    length: libc::c_ulong,
    is_null: ffi::my_bool,
    is_truncated: Option<ffi::my_bool>,
}

impl BindData {
    unsafe fn mysql_bind(&mut self) -> ffi::MYSQL_BIND {
        let mut bind: ffi::MYSQL_BIND = mem::zeroed();
        bind.buffer_type = self.tpe;
        bind.buffer = self.bytes.as_mut_ptr() as *mut libc::c_void;
        bind.buffer_length = self.bytes.capacity() as libc::c_ulong;
        bind.length = &mut self.length;
        bind.is_null = &mut self.is_null;

        if let Some(ref mut is_truncated) = self.is_truncated {
            bind.error = is_truncated;
        }

        bind
    }
}

impl Statement {
    pub(super) fn input_bind(&mut self, mut input_binds: Binds) -> QueryResult<()> {
        {
                    let mut binds = self
                       .data
                       .iter_mut()
                       .map(|x| unsafe { x.mysql_bind() })
                      .collect::<Vec<_>>();
                   let bind_ptr = binds.as_mut_ptr(); // you cannot assume if `input_binds` is  read or not after this point anymore
                  some_function_that_reads_this_pointer_at_a_later_point_in_time(bind_ptr)
        }
        self.input_binds = Some(input_binds);
        self.did_an_error_occur()
    }
}

