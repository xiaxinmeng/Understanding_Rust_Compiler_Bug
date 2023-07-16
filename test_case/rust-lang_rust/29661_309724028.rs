
fn interp<X,Y>(x:X, (x0,y0):(X,Y), (x1,y1):(X,Y))->Y 
	where X:HasOperatorsWith<Y>
{	((x-x0)/(x1-x0)) * (y1-y0) + y0
}

// Helper trait implies existance of operators between types Self,B
// such that addition/subtraction , multiplication/division produce intermediates
// but inverse operations cancel out.
// even differences may be other types, e.g. Unsigned minus Unsigned -> Signed
// Point minus Point -> Offset

pub trait HasOperatorsWith<B> {
	type Diff=<Self as Sub<B>>::Output;
	type ProdWith=<Self as Mul<B>>::Output;
	type DivBy=<Self as Div<B>>::Output;
	type Ratio=<Self as Div<Self>>::Output;
	assume <Diff as Div<Diff>>::Output= Ratio;
	type Square = <Self as Mul<B>> :: Ouput;
	assume <ProdWith as Mul<DivBy> >::Output = Self // multiplication and division must be inverses
	assume <Sum as Mul<DivBy> >::Output = Self
	assume <Square as Sqrt<> > :: Output = Self
	assume <Self as Add<Diff>> :: Output= Self   // addition and subtraction must be inverses
	// etc..
}


// implementor - so long as all those operators exist, consider 'HasOperatorsWith' to exist
// currently you must write another to instantiate it

impl <....> HasOperatorsWith<X> for Y  where /* basically repeat all that above*/
    

