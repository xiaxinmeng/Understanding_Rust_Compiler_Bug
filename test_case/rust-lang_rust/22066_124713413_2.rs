 rust
use std::marker::PhantomData; 

pub struct GILGuard;
trait PythonObject<'a> {
    fn dummy() -> &'a i32;
}

#[derive(Copy, Clone)]                                                          
pub struct Python<'p>(PhantomData<&'p GILGuard>);                               

pub struct PyErr<'p> {                                                          
    pub ptype : PyObject<'p>,                                                   
}  

type PyResult<'p, T> = Result<T, PyErr<'p>>;

pub struct PyObject<'a> {
    dummy : &'a i32
}

impl <'p> PyObject<'p> { 
    pub fn cast_into<T>(self) -> Result<T, PythonObjectDowncastError<'p>> where T: PythonObjectWithCheckedDowncast<'p> {
        PythonObjectWithCheckedDowncast::downcast_from(self)                    
    }   
}

pub struct PythonObjectDowncastError<'p>(pub Python<'p>);                       
pub trait PythonObjectWithCheckedDowncast<'p> : PythonObject<'p> {              
    fn downcast_from(PyObject<'p>) -> Result<Self, PythonObjectDowncastError<'p>>;
}   

impl <'p> PythonObjectWithCheckedDowncast<'p> for PyObject<'p> {                
    #[inline]                                                                   
    fn downcast_from(obj: PyObject<'p>) -> Result<PyObject<'p>, PythonObjectDowncastError<'p>> {
        Ok(obj)                                                                 
    }
}

impl <'p> PythonObject<'p> for PyObject<'p> {                                   
    fn dummy() -> &'p i32 { &7 }
}

pub trait PyExtractor<'python, 'source, 'prepared> {                            
    type Prepared;                                                              
    fn extract<T>(prepared: &'prepared Self::Prepared) -> PyResult<'python, T>  
        where T: PythonObjectWithCheckedDowncast<'python> ;                     
}                                                                               

pub struct PyObjectExtractor;                                                   

impl <'python, 'source, 'prepared> PyExtractor<'python, 'source, 'prepared>     
    for PyObjectExtractor {                                                     
    type Prepared = &'source PyObject<'python>;                                 

    #[inline]                                                                   
    fn extract<T>(&&ref obj: &'prepared Self::Prepared) -> PyResult<'python, T> 
        where T: PythonObjectWithCheckedDowncast<'python> {                     
        Ok(try!(obj.clone().cast_into()))                                       
    }                                                                           
}                                                                               

pub trait ExtractPyObject<'python, 'source, 'prepared> {                        
    type Prepared;                                                              

    fn prepare_extract(obj: &'source PyObject<'python>) -> PyResult<'python, Self::Prepared>;

    fn extract<E: PyExtractor<'python, 'source, 'prepared>>(prepared: &'prepared Self::Prepared) -> PyResult<'python, Self>;
}                                                                               

impl <'python, 'source, 'prepared, T> ExtractPyObject<'python, 'source, 'prepared>
    for T where T: PythonObjectWithCheckedDowncast<'python> {                   

    type Prepared = &'source PyObject<'python>;                                 

    #[inline]                                                                   
    fn prepare_extract(obj: &'source PyObject<'python>) -> PyResult<'python, Self::Prepared> {
        Ok(obj)                                                                 
    }                                                                           

    #[inline]                                                                   
    fn extract<E: PyExtractor<'python, 'source, 'prepared>>(&&ref obj: &'prepared Self::Prepared) -> PyResult<'python, T> {
        E::extract::<Self::Prepared>(&obj)                                      
    }                                                                           
}
