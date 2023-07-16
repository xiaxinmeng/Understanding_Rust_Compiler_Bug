ada
function Area(C: Circle)
   with Pre => C.X_Coord – C.Radius > 0.0,
        Post => Area'Result > 3.1 * C.Radius**2 and
                Area'Result < 3.2 * C.Radius**2;
