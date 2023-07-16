
> Add unused code pass
> 
> In order to find more examples of dead code we must create liveness variables
> and scan after type resolution. In this case, if a function is unused and it calls
> another function the 2nd function is now unused since the caller is not used etc.
> This is a WIP but demonstrates the pattern we can follow to add more dead code
> analysis warnings like rustc does.
> 
> Addresses #...
> 