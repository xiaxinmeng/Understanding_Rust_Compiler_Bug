rust
myfile.rs:

/// my function does blah blah blah. here is a picture:
/// <div>
/// <img src="data:image/png;base64,
/// iVBORw0KGgetcetcetc                       <-- copy/paste myfile.b64 here
/// ....                                      <-- several dozen lines of myfile.b64
/// JDIOIDondio00778==                        <-- last line of myfile.b64
/// ">
/// </div>
fn myfunction(x:u64)->bool { x<5 }
