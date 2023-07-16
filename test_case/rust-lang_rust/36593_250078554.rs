
    repeat until fixed point:
        for every function in SCC:
            try to inline all call-sites in function
            if function changed:
                do post-inlining cleanup passes
