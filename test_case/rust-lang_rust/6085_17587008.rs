
22:05 < strcat> brson: well, ideally not an enum
22:05 <@brson> yichoi: graydon could disagree though. I'm not sure what his vision for that module is and 
               he usually likes less abstraction than I
22:05 < strcat> we just need a flag type with | overloaded
22:05 < strcat> and some constants
22:05 < strcat> casting to an int to OR them would be annoying
